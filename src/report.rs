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

use anyhow::Context;
use chrono::Local;
use rust_ev_verifier_application_lib::{
    RunInformation,
    report::{
        ReportConfigBuilder, ReportData, ReportInformationTrait, ReportOutputDataMetaDataBuilder,
        ReportOutputFileOptions, generate_files_from_json,
    },
};
use rust_ev_verifier_lib::VerifierConfig;
use std::path::Path;
use tracing::{error, info, instrument};

pub fn generate_report(
    run_info: &RunInformation,
    config: &'static VerifierConfig,
) -> anyhow::Result<()> {
    let report_title = format!(
        "E-Voting Verifier Report {}",
        run_info.verification_period().as_ref().unwrap(),
    );

    let now = Local::now();

    let report_data = ReportData::new(
        ReportConfigBuilder::default()
            .title(&report_title)
            .date_time(now.format("%d.%m.%Y %H:%M:%S").to_string().as_str())
            .tab_size(config.txt_report_tab_size())
            .fromat_date(config.report_format_date().to_string())
            .build()
            .context("Error creating the ReportConfig")?,
        &run_info,
    );

    let seed = run_info.runner_information().seed.as_ref().unwrap();

    let filename_without_extension = format!(
        "e_voting_verifier_report_{}_{}_{}",
        run_info.verification_period().as_ref().unwrap(),
        seed,
        now.format("%Y%m%d_%H%M%S")
    );

    let report_dir_path = config.report_dir_path();

    let json_path = report_dir_path.join(format!("{}.json", filename_without_extension));
    std::fs::write(
        &json_path,
        report_data
            .to_json()
            .context("Error generating JSON report data")?,
    )
    .with_context(|| format!("Error writing JSON report data to {}", json_path.display()))?;
    info!(
        "JSON report written to {}. It can be use to regenerate the reports using the command report",
        json_path.display()
    );

    let options = ReportOutputFileOptions::generate_from_config(
        &filename_without_extension,
        &report_dir_path,
        config,
    )?;

    match report_data.generate_files(
        ReportOutputDataMetaDataBuilder::default()
            .title(&report_title)
            .date_time(now.format("%d.%m.%Y - %H:%M:%S").to_string().as_str())
            .seed(seed)
            .build()
            .context("Error building the report output metadata")?,
        options,
    ) {
        errors if errors.is_empty() => {
            info!("Reports generated in {}", report_dir_path.display());
        }
        errors => {
            for error in errors {
                error!("{:?}", error);
            }
        }
    };
    Ok(())
}

/// Execute the report generation from a JSON report file
#[instrument(skip(config))]
pub fn execute_report(input: &Path, config: &VerifierConfig) -> anyhow::Result<()> {
    let json_str = std::fs::read_to_string(input).map_err(|e| {
        anyhow::anyhow!("Error reading report json file {}: {}", input.display(), e)
    })?;

    let target_dir = config.report_dir_path();

    let options = ReportOutputFileOptions::generate_from_config(
        &input.file_stem().unwrap().to_str().unwrap(),
        &target_dir,
        config,
    )?;

    match generate_files_from_json(&json_str, options) {
        errors if errors.is_empty() => {
            info!("Reports generated in {}", target_dir.display());
        }
        errors => {
            for error in errors {
                error!("{:?}", error);
            }
        }
    };

    Ok(())
}
