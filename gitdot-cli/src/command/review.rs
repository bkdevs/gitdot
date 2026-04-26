mod amend;
mod checkout;
mod create;
mod update;

use clap::{Args, Subcommand};

use crate::{config::UserConfig, git::GitWrapper};

#[derive(Args, Debug)]
pub struct ReviewArgs {
    #[command(subcommand)]
    pub command: ReviewCommand,
}

#[derive(Subcommand, Debug)]
pub enum ReviewCommand {
    /// Create a review
    New {
        /// Review message; first line is the title, remaining lines are the description
        #[arg(short = 'm', long = "message")]
        message: Option<String>,
    },

    /// Checkout a commit from the review
    Checkout,

    /// Amend changes into the checked-out commit and rebase
    Amend,

    /// Update an existing review
    Update,
}

impl ReviewCommand {
    pub async fn execute(&self, config: UserConfig) -> anyhow::Result<()> {
        let git = GitWrapper::new();
        match self {
            ReviewCommand::New { message } => {
                create::create_review(config, &git, message.clone()).await
            }
            ReviewCommand::Checkout {} => checkout::checkout_review(config, &git).await,
            ReviewCommand::Amend {} => amend::amend_review(config, &git).await,
            ReviewCommand::Update {} => update::update_review(config, &git).await,
        }
    }
}
