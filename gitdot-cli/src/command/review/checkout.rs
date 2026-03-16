use std::io::{self, Write};

use anyhow::{Context, bail};
use tokio::process::Command;

use super::get_default_branch;
use crate::config::UserConfig;

pub async fn checkout_review(_config: UserConfig) -> anyhow::Result<()> {
    let default_branch = get_default_branch().await?;

    let output = Command::new("git")
        .args([
            "log",
            &format!("origin/{}..HEAD", default_branch),
            "--oneline",
        ])
        .output()
        .await
        .context("Failed to run git log")?;

    if !output.status.success() {
        bail!("Failed to list commits");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let commits: Vec<(&str, &str)> = stdout
        .lines()
        .filter_map(|line| {
            let (hash, subject) = line.split_once(' ')?;
            Some((hash, subject))
        })
        .collect();

    if commits.is_empty() {
        bail!("No commits ahead of origin/{}", default_branch);
    }

    let selected = if commits.len() == 1 {
        0
    } else {
        println!("Commits ahead of origin/{}:", default_branch);
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

    let (hash, subject) = commits[selected];

    let status = Command::new("git")
        .args(["checkout", hash])
        .status()
        .await
        .context("Failed to run git checkout")?;

    if !status.success() {
        bail!("Failed to checkout {}", hash);
    }

    println!("Checked out: {} {}", hash, subject);

    Ok(())
}
