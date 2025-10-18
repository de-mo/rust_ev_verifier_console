// Copyright © 2025 Denis Morel
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License and
// a copy of the GNU General Public License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

use crate::{VerifierSubCommand, report::generate_report};
use anyhow::Context;
use rust_ev_verifier_application_lib::{
    ExtractDataSetResults, RunInformation, RunParallel, Runner,
};
use rust_ev_verifier_lib::{
    VerifierConfig,
    verification::{VerificationMetaDataList, VerificationPeriod},
};
use std::sync::{Arc, RwLock};
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
    let tally_zip_file = sub_command.tally_zip.as_deref();
    info!("Start extraction");
    let extracted = ExtractDataSetResults::extract_datasets(
        *period,
        context_zip_file,
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
        .prepare_data_for_start(*period, &metadata, &exclusion)
        .context("Error preparing data for running")?;
    run_information.add_extracted_information(&extracted);
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
    runner
        .run_all(&metadata)
        .context("error running the tests")?;
    info!("All verifications finished");

    let run_info = run_information_lock.read().unwrap();
    generate_report(&run_info, config).context("Error generating the reports")?;
    info!("Verifier finished");
    Ok(())
}
