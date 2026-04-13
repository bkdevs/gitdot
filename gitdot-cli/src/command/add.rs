use clap::Args;

use crate::git::GitWrapper;

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Commit message
    message: String,
}

impl AddArgs {
    pub async fn execute(&self) -> anyhow::Result<()> {
        let git = GitWrapper::new();
        git.add_all().await?;
        if let Ok(last) = git.last_commit_message().await {
            if last == self.message {
                return git.commit_amend_no_edit().await;
            }
        }
        git.commit_allow_empty(&self.message).await
    }
}
