use anyhow::ensure;
use rust_ev_verifier_lib::{dataset::DatasetMetadata, VerifierConfig};
use std::path::Path;
use tracing::{info, instrument};

/// Execute the verifications, starting the runner
#[instrument(skip(password, config))]
pub fn execute_extract(
    input: &Path,
    password: &str,
    dataset_type_str: &str,
    config: &VerifierConfig,
) -> anyhow::Result<()> {
    ensure!(
        ["context", "setup", "tally"].contains(&dataset_type_str),
        "not correct dataset type: only context, setup or tally allowed"
    );
    let target_dir = config.create_dataset_dir_path();
    info!(
        "Start extracting file {}",
        input.as_os_str().to_str().unwrap(),
    );
    let meta_data = DatasetMetadata::extract_dataset_str_with_inputs(
        dataset_type_str,
        input,
        password,
        &target_dir,
        &config.zip_temp_dir_path(),
    )?;
    info!(
        "Successfully extraction {} from file {} in directory {}. (Fingerprint: {})",
        meta_data.kind().as_ref(),
        input.as_os_str().to_str().unwrap(),
        meta_data.extracted_dir_path().as_os_str().to_str().unwrap(),
        meta_data.fingerprint(),
    );
    Ok(())
}
