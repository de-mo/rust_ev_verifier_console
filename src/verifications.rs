use std::sync::{Arc, RwLock};

use crate::VerifierSubCommand;
use anyhow::Context;
use rust_ev_verifier_lib::{
    application_runner::{
        report::ReportData, run_information::RunInformation, ExtractDataSetResults, RunParallel,
        Runner,
    },
    verification::{VerificationMetaDataList, VerificationPeriod},
    VerifierConfig,
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
    let mut run_information = RunInformation::new(config);
    run_information
        .prepare_data_for_running(*period, &metadata, &exclusion, &extracted)
        .context("Error preparing data for running")?;
    let run_information_lock = Arc::new(RwLock::new(run_information));
    let run_information_lock_before_run = run_information_lock.clone();
    let run_information_lock_after_run = run_information_lock.clone();
    let run_information_lock_before_verif = run_information_lock.clone();
    let run_information_lock_after_verif = run_information_lock.clone();
    let mut runner = Runner::new(
        extracted.location(),
        period,
        &metadata,
        exclusion.as_slice(),
        RunParallel,
        config,
        move |start_time| {
            trace!("Before running start");
            let mut r_info_mut = run_information_lock_before_run.write().unwrap();
            r_info_mut.start_running(&start_time);
            trace!("After running start");
        },
        move |id| {
            trace!("Start before verification for {}", id);
            let mut r_info_mut = run_information_lock_before_verif.write().unwrap();
            r_info_mut.start_verification(id);
            trace!("End before verification for {}", id);
        },
        move |verif_information| {
            trace!("Start after verification for {}", &verif_information.id);
            let mut r_info_mut = run_information_lock_after_verif.write().unwrap();
            r_info_mut.finish_verification(&verif_information);
            trace!("End before verification for {}", &verif_information.id);
        },
        move |run_info| {
            trace!("Before running finished");
            let mut r_info_mut = run_information_lock_after_run.write().unwrap();
            r_info_mut.finish_runner(&run_info);
            trace!("After running finished");
        },
    )
    .context("Error creating the runner")?;
    let verif_directory = runner.verification_directory().clone();
    runner
        .run_all(&metadata)
        .context("error running the tests")?;
    let run_info_read = run_information_lock.read().unwrap();
    let report = ReportData::new(&run_info_read);
    info!("Report: \n{}", report.to_string());
    info!("Verifier finished");
    Ok(())
}
