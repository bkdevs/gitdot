mod install;
mod register;
mod run;
mod start;
mod stop;

use install::install;
use register::register;
use run::run;
use start::start;
use stop::stop;

use crate::config::Config;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Register a new runner
    Register {},

    /// Synchronously run the runner (TK comment out somehow or hide?)
    Run {},

    /// Install the runner daemon
    Install {},

    /// Start the runner daemon
    Start {},

    /// Stop the runner daemon
    Stop {},
}

impl Commands {
    pub async fn execute(&self, mut config: Config) -> anyhow::Result<()> {
        match self {
            Commands::Register {} => register(config).await,
            Commands::Run {} => run(config).await,
            Commands::Install {} => install(config).await,
            Commands::Start {} => start(config).await,
            Commands::Stop {} => stop(config).await,
        }
    }
}
