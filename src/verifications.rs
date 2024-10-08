use anyhow::Context;
use rust_verifier::{
    application_runner::{
        no_action_after_fn, no_action_before_fn, ExtractDataSetResults, RunParallel, Runner,
    },
    verification::{VerificationMetaDataList, VerificationPeriod},
    Config as VerifierConfig,
};
use tracing::{info, instrument};

use crate::VerifierSubCommand;

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
    let mut runner = Runner::new(
        extracted.location(),
        period,
        &metadata,
        &sub_command.exclude,
        RunParallel,
        config,
        no_action_before_fn,
        no_action_after_fn,
    )
    .context("Error creating the runner")?;
    runner
        .run_all(&metadata)
        .context("error running the tests")?;
    info!("Verifier finished");
    Ok(())
}
