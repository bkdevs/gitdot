mod create;

use clap::{Args, Subcommand};

use crate::config::UserConfig;

#[derive(Args, Debug)]
pub struct ReviewArgs {
    #[command(subcommand)]
    pub command: ReviewCommand,
}

#[derive(Subcommand, Debug)]
pub enum ReviewCommand {
    /// Create a review
    Create,
}

impl ReviewCommand {
    pub async fn execute(&self, config: UserConfig) -> anyhow::Result<()> {
        match self {
            ReviewCommand::Create {} => create::create_review(config).await,
        }
    }
}
