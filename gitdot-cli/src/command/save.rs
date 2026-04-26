use clap::Args;

use crate::git::GitWrapper;

#[derive(Args, Debug)]
pub struct SaveArgs {
    /// Commit message
    message: Option<String>,
}

impl SaveArgs {
    pub async fn execute(&self) -> anyhow::Result<()> {
        let git = GitWrapper::new();
        git.add_all().await?;
        let Some(ref message) = self.message else {
            return git.commit_amend_no_edit().await;
        };
        if let Ok(last) = git.last_commit_message().await {
            if &last == message {
                return git.commit_amend_no_edit().await;
            }
        }
        git.commit_allow_empty(message).await
    }
}
