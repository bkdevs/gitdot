mod config;
mod install;
mod run;
mod start;
mod stop;
mod verify;

use config::config as print_config;
use install::install;
use run::run;
use start::start;
use stop::stop;
use verify::verify;

use crate::config::RunnerConfig;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct RunnerArgs {
    #[command(subcommand)]
    pub command: RunnerCommand,
}

#[derive(Subcommand, Debug)]
pub enum RunnerCommand {
    /// Install the runner daemon
    Install {},

    /// Synchronously run the runner
    Run {},

    /// Start the runner daemon
    Start {},

    /// Stop the runner daemon
    Stop {},

    /// Edit runner configuration
    Config {},

    /// Verify runner configuration
    Verify {},
}

impl RunnerCommand {
    pub async fn execute(&self, config: RunnerConfig) -> anyhow::Result<()> {
        match self {
            RunnerCommand::Install {} => install(config).await,
            RunnerCommand::Run {} => run(config).await,
            RunnerCommand::Start {} => start().await,
            RunnerCommand::Stop {} => stop().await,
            RunnerCommand::Config {} => print_config().await,
            RunnerCommand::Verify {} => verify(config).await,
        }
    }
}
