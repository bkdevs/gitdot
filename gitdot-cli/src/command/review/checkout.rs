use std::io::{self, Write};

use anyhow::{Context, bail};
use tokio::fs;

use crate::{config::UserConfig, git::GitWrapper};

pub async fn checkout_review(_config: UserConfig, git: &GitWrapper) -> anyhow::Result<()> {
    let branch = git.current_branch().await?;
    if branch.is_empty() {
        bail!("Not currently on a branch");
    }

    let commits = git.log_oneline(&format!("origin/{}..HEAD", branch)).await?;

    if commits.is_empty() {
        bail!("No commits ahead of origin/{}", branch);
    }

    let selected = if commits.len() == 1 {
        0
    } else {
        println!("Commits ahead of origin/{}:", branch);
        for (i, (hash, subject)) in commits.iter().enumerate() {
            println!("  [{}] {} {}", i + 1, &hash[..7.min(hash.len())], subject);
        }

        print!("Select a commit [1-{}]: ", commits.len());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice: usize = input.trim().parse().context("Invalid selection")?;

        if choice < 1 || choice > commits.len() {
            bail!("Selection out of range");
        }

        choice - 1
    };

    let (hash, subject) = &commits[selected];

    // Save current branch name so `gdot review amend` knows where to rebase
    let git_dir = git.git_dir().await?;
    fs::write(git_dir.join("gdot-review-branch"), &branch).await?;

    git.checkout(hash).await?;

    println!("Checked out: {} {}", hash, subject);

    Ok(())
}
