//! Shell command implementation
//!
//! For help:
//! ```shell
//! rust_verifier_console --help
//! ```

mod extract;
mod logger;
mod verifications;

use anyhow::anyhow;
use extract::execute_extract;
use lazy_static::lazy_static;
use log::{error, LevelFilter};
use logger::init_logger;
use rust_verifier::{verification::VerificationPeriod, Config as VerifierConfig};
use std::path::PathBuf;
use structopt::StructOpt;
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
    pub context_zip: PathBuf,

    #[structopt(long, parse(from_os_str))]
    pub setup_zip: Option<PathBuf>,

    #[structopt(long, parse(from_os_str))]
    pub tally_zip: Option<PathBuf>,
}

/// Enum with the possible subcommands
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
        input: PathBuf,
        #[structopt(short, long)]
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
        error!("{}", e);
        anyhow!(e)
    })?;
    let password = dotenvy::var("VERIFIER_DATASET_PASSWORD").map_err(|e| {
        error!(
            "Password (VERIFIER_DATASET_PASSWORD) not found in .env {}",
            e
        );
        anyhow!(e)
    })?;
    init_logger(&CONFIG, LevelFilter::Debug, true);
    if let Err(e) = execute_command(&password) {
        error!("{}", e)
    }
    Ok(())
}
