use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::VerifierSubCommand;
use anyhow::Context;
use rust_ev_verifier_lib::{
    application_runner::{
        report::ReportData, ExtractDataSetResults, RunParallel, Runner, RunnerInformation,
    },
    file_structure::VerificationDirectoryTrait,
    verification::{
        ManualVerifications, VerificationMetaDataList, VerificationPeriod, VerificationStatus,
    },
    Config as VerifierConfig,
};
use tracing::{info, instrument, trace};

/// Execute the verifications, starting the runner
#[instrument(skip(password, config))]
pub fn execute_verifications(
    period: &VerificationPeriod,
    sub_command: &VerifierSubCommand,
    password: &str,
    config: &'static VerifierConfig,
) -> anyhow::Result<()> {
    let context_zip_file = &sub_command.context_zip;
    let setup_zip_file = sub_command.setup_zip.as_deref();
    let tally_zip_file = sub_command.tally_zip.as_deref();
    info!("Start extraction");
    let extracted = ExtractDataSetResults::extract_datasets(
        *period,
        context_zip_file,
        setup_zip_file,
        tally_zip_file,
        password,
        config,
    )
    .context("Problem with extraction")?;
    info!("extraction finished");
    info!("Start Verifier for {}", period.as_ref());
    let metadata = VerificationMetaDataList::load(config.get_verification_list_str()).unwrap();
    let exclusion = sub_command
        .exclude
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let verifications_not_finished = Arc::new(RwLock::new(vec![]));
    let verifications_not_finished_cloned = verifications_not_finished.clone();
    let verifications_not_finished_final = verifications_not_finished.clone();
    let verifications_with_errors_and_failures = Arc::new(RwLock::new(HashMap::new()));
    let verifications_with_errors_and_failures_final =
        verifications_with_errors_and_failures.clone();
    let runner_information = Arc::new(RwLock::new(RunnerInformation::default()));
    let runner_information_final = runner_information.clone();
    let mut runner = Runner::new(
        extracted.location(),
        period,
        &metadata,
        exclusion.as_slice(),
        RunParallel,
        config,
        move |id| {
            trace!("Start before verification for {}", id);
            let mut verif_not_finished_mut = verifications_not_finished.write().unwrap();
            verif_not_finished_mut.push(id.to_string());
            trace!("End before verification for {}", id);
        },
        move |verif_information| {
            trace!("Start after verification for {}", &verif_information.id);
            let mut verif_not_finished_mut = verifications_not_finished_cloned.write().unwrap();
            match verif_not_finished_mut
                .iter()
                .position(|id| id == &verif_information.id)
            {
                Some(pos) => {
                    let _ = verif_not_finished_mut.remove(pos);
                }
                None => {}
            }
            if verif_information.status != VerificationStatus::FinishedSuccessfully {
                let mut verifs_res_mut = verifications_with_errors_and_failures.write().unwrap();
                verifs_res_mut.insert(
                    verif_information.id.clone(),
                    (
                        verif_information.errors.len() as u8,
                        verif_information.failures.len() as u8,
                    ),
                );
            }
            trace!("End before verification for {}", &verif_information.id);
        },
        move |run_info| {
            trace!("After running start");
            let mut r_info_mut = runner_information.write().unwrap();
            r_info_mut.start_time = run_info.start_time.clone();
            r_info_mut.duration = run_info.duration;
            trace!("After running end");
        },
    )
    .context("Error creating the runner")?;
    let verif_directory = runner.verification_directory().clone();
    runner
        .run_all(&metadata)
        .context("error running the tests")?;
    let manual_verif = ManualVerifications::new(
        *period,
        &verif_directory,
        config,
        verifications_not_finished_final.read().unwrap().clone(),
        verifications_with_errors_and_failures_final
            .read()
            .unwrap()
            .clone(),
        exclusion,
    )
    .context("Error generating manual verfications")?;
    let run_info_read = runner_information_final.read().unwrap();
    let report = ReportData::new(
        verif_directory.path(),
        period,
        &manual_verif,
        &extracted,
        &run_info_read,
    );
    info!("Report: \n{}", report.to_string());
    info!("Verifier finished");
    Ok(())
}
