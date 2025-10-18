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

use anyhow::ensure;
use rust_ev_verifier_lib::{VerifierConfig, dataset::DatasetMetadata};
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
