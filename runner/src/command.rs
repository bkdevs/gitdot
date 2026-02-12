mod register;

use clap::{Parser, Subcommand};

pub use register::register;

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
