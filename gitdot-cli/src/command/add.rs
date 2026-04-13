use clap::Args;

use crate::git::GitWrapper;

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Commit message
    message: Option<String>,
}

impl AddArgs {
    pub async fn execute(&self) -> anyhow::Result<()> {
        let git = GitWrapper::new();
        git.add_all().await?;
        git.commit_allow_empty(self.message.as_deref().unwrap_or(""))
            .await
    }
}
