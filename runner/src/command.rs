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
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install the runner daemon
    Install {},

    /// Synchronously run the runner (TK comment out somehow or hide?)
    Run {},

    /// Start the runner daemon
    Start {},

    /// Stop the runner daemon
    Stop {},
}

impl Commands {
    pub async fn execute(&self, config: Config) -> anyhow::Result<()> {
        match self {
            Commands::Install {} => install(config).await,
            Commands::Run {} => run(config).await,
            Commands::Start {} => start(config).await,
            Commands::Stop {} => stop(config).await,
        }
    }
}
