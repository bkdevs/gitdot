mod format;
mod init;
mod lint;
mod run;

use format::format;
use init::init;
use lint::lint;
use run::run;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct CiArgs {
    #[command(subcommand)]
    pub command: CiCommand,
}

#[derive(Subcommand, Debug)]
pub enum CiCommand {
    /// Create a new .gitdot-ci.toml file
    Init {},

    /// Lint .gitdot-ci.toml
    Lint {},

    /// Format .gitdot-ci.toml
    Format {},

    /// Run a workflow in .gitdot-ci.toml
    Run {},
}

impl CiCommand {
    pub async fn execute(&self) -> anyhow::Result<()> {
        match self {
            CiCommand::Init {} => init().await,
            CiCommand::Lint {} => lint().await,
            CiCommand::Format {} => format().await,
            CiCommand::Run {} => run().await,
        }
    }
}
