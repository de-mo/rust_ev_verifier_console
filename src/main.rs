//! Shell command implementation
//!
//! For help:
//! ```shell
//! rust_ev_verifier_lib_console --help
//! ```

mod extract;
mod subscriber;
mod verifications;

use anyhow::anyhow;
use extract::execute_extract;
use lazy_static::lazy_static;
use rust_ev_verifier_lib::{VerifierConfig, verification::VerificationPeriod};
use std::path::PathBuf;
use structopt::StructOpt;
use subscriber::init_subscriber;
use tracing::{error, info, instrument};
use verifications::execute_verifications;

lazy_static! {
    static ref CONFIG: VerifierConfig = VerifierConfig::new(".");
}

/// Specification of the sub commands tally and setup
#[derive(Debug, PartialEq, StructOpt)]
#[structopt()]
pub struct VerifierSubCommand {
    #[structopt(long)]
    /// Exclusion of verifications.
    /// Use the id of the verification. Many separated by blanks. E.g. --exclude 02.02 05.05
    pub exclude: Vec<String>,

    #[structopt(long, parse(from_os_str))]
    /// Path to the context zip file.
    pub context_zip: PathBuf,

    #[structopt(long, parse(from_os_str))]
    /// Path to the tally zip file.
    /// Mandatory for tally
    pub tally_zip: Option<PathBuf>,
}

#[derive(Debug, PartialEq, StructOpt)]
#[structopt()]
pub enum SubCommands {
    #[structopt()]
    /// Verify the setup configuration
    Setup(VerifierSubCommand),

    #[structopt()]
    /// Verify the tally configuration
    Tally(VerifierSubCommand),

    #[structopt()]
    /// Extraction of the zip
    Extract {
        #[structopt(short, long, parse(from_os_str))]
        /// The path to the zip file
        input: PathBuf,
        #[structopt(short, long)]
        /// The type of the dataset.
        /// Only values "context", "setup", "tally" are valid
        dataset_type: String,
    },
}

/// Main command
#[derive(Debug, StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
/// E-Voting Verifier
/// Verifier for E-Voting System of Swiss Post
pub struct VerifiyCommand {
    #[structopt(subcommand)]
    pub sub: SubCommands,
}

/// Execute the command
/// This is the main method called from the console
///
/// # return
/// * Nothing if the execution runs correctly
/// * [anyhow::Result] with the related error by a problem
#[instrument(skip(password))]
fn execute_command(password: &str) -> anyhow::Result<()> {
    match VerifiyCommand::from_args().sub {
        SubCommands::Setup(c) => {
            execute_verifications(&VerificationPeriod::Setup, &c, password, &CONFIG)
        }
        SubCommands::Tally(c) => {
            execute_verifications(&VerificationPeriod::Tally, &c, password, &CONFIG)
        }
        SubCommands::Extract {
            input,
            dataset_type,
        } => execute_extract(&input, password, &dataset_type, &CONFIG),
    }
}

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv().map_err(|e| {
        let error = anyhow!(format!("Error reading .env file: {e}"));
        error
    })?;

    let log_file_path = CONFIG.log_file_path();
    let log_dir_path = log_file_path.parent().unwrap();
    if !log_dir_path.exists() {
        std::fs::create_dir_all(log_dir_path)?;
    }

    let _guards = init_subscriber(&CONFIG);

    info!(
        "Starting the verifier Console (Version: {})",
        env!("CARGO_PKG_VERSION")
    );

    let password = CONFIG.decrypt_password().map_err(|e| {
        error!("Error reading password: {}", e);
        anyhow!(e)
    })?;
    if let Err(e) = execute_command(&password) {
        error!("{:?}", e)
    }
    Ok(())
}
