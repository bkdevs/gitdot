mod install;
mod run;
mod start;
mod stop;

use install::install;
use run::run;
use start::start;
use stop::stop;

use crate::config::Config;
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
    /// Install the runner daemon
    Install {},

    /// Synchronously run the runner (TK comment out somehow or hide?)
    Run {},

    /// Start the runner daemon
    Start {},

    /// Stop the runner daemon
    Stop {},
}

impl CiCommand {
    pub async fn execute(&self, config: Config) -> anyhow::Result<()> {
        match self {
            CiCommand::Install {} => install(config).await,
            CiCommand::Run {} => run(config).await,
            CiCommand::Start {} => start(config).await,
            CiCommand::Stop {} => stop(config).await,
        }
    }
}
