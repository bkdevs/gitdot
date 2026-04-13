use std::{path::PathBuf, process::Stdio};

use anyhow::{Context, bail};
use tokio::process::Command;

pub struct GitWrapper;

impl GitWrapper {
    pub fn new() -> Self {
        Self
    }

    // --- Primitives ---

    pub async fn remote_url(&self, name: &str) -> anyhow::Result<String> {
        self.run(&["remote", "get-url", name]).await
    }

    pub async fn default_branch(&self) -> anyhow::Result<String> {
        let remote_head = self
            .run(&["symbolic-ref", "--short", "refs/remotes/origin/HEAD"])
            .await?;
        Ok(remote_head
            .rsplit('/')
            .next()
            .unwrap_or(&remote_head)
            .to_string())
    }

    pub async fn git_dir(&self) -> anyhow::Result<PathBuf> {
        let dir = self.run(&["rev-parse", "--git-dir"]).await?;
        Ok(PathBuf::from(dir))
    }

    pub async fn rev_parse(&self, rev: &str) -> anyhow::Result<String> {
        self.run(&["rev-parse", rev]).await
    }

    pub async fn current_branch(&self) -> anyhow::Result<String> {
        self.run(&["branch", "--show-current"]).await
    }

    pub async fn last_commit_message(&self) -> anyhow::Result<String> {
        self.run(&["log", "-1", "--format=%s"]).await
    }

    pub async fn log_oneline(&self, range: &str) -> anyhow::Result<Vec<(String, String)>> {
        let stdout = self.run(&["log", range, "--oneline"]).await?;
        Ok(stdout
            .lines()
            .filter_map(|line| {
                let (hash, subject) = line.split_once(' ')?;
                Some((hash.to_string(), subject.to_string()))
            })
            .collect())
    }

    // --- Operations ---

    pub async fn pull_rebase(&self, branch: &str) -> anyhow::Result<()> {
        self.run_silent(&["pull", "origin", branch, "--rebase"])
            .await
            .with_context(|| format!("Failed to rebase onto origin/{}", branch))
    }

    pub async fn push_refspec(&self, refspec: &str) -> anyhow::Result<String> {
        let output = Command::new("git")
            .args(["push", "origin", refspec])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git push")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("{}", stderr.trim());
            bail!("Failed to push to {}", refspec);
        }

        Ok(String::from_utf8_lossy(&output.stderr).to_string())
    }

    pub async fn add_all(&self) -> anyhow::Result<()> {
        self.run_status(&["add", "-A"])
            .await
            .context("Failed to stage changes")
    }

    pub async fn commit_amend_no_edit(&self) -> anyhow::Result<()> {
        self.run_status(&["commit", "--amend", "--no-edit"])
            .await
            .context("Failed to amend commit")
    }

    pub async fn commit_allow_empty(&self, message: &str) -> anyhow::Result<()> {
        self.run_status(&["commit", "--allow-empty-message", "-m", message])
            .await
            .context("Failed to commit")
    }

    pub async fn rebase_onto(
        &self,
        new_base: &str,
        old_base: &str,
        branch: &str,
    ) -> anyhow::Result<()> {
        self.run_status(&["rebase", "--onto", new_base, old_base, branch])
            .await
            .context("Rebase failed. Resolve conflicts and run `git rebase --continue`.")
    }

    pub async fn checkout(&self, rev: &str) -> anyhow::Result<()> {
        self.run_status(&["checkout", rev])
            .await
            .with_context(|| format!("Failed to checkout {}", rev))
    }

    // --- Internal ---

    /// Runs git with args, checks status, returns trimmed stdout.
    async fn run(&self, args: &[&str]) -> anyhow::Result<String> {
        let output = Command::new("git")
            .args(args)
            .output()
            .await
            .with_context(|| format!("Failed to run `git {}`", args.join(" ")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("`git {}` failed: {}", args.join(" "), stderr.trim());
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    /// Runs git with args, checks status, inherits stdio (visible to user).
    async fn run_status(&self, args: &[&str]) -> anyhow::Result<()> {
        let status = Command::new("git")
            .args(args)
            .status()
            .await
            .with_context(|| format!("Failed to run `git {}`", args.join(" ")))?;

        if !status.success() {
            bail!("`git {}` failed", args.join(" "));
        }

        Ok(())
    }

    /// Runs git with args, checks status, suppresses all output.
    async fn run_silent(&self, args: &[&str]) -> anyhow::Result<()> {
        let status = Command::new("git")
            .args(args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .with_context(|| format!("Failed to run `git {}`", args.join(" ")))?;

        if !status.success() {
            bail!("`git {}` failed", args.join(" "));
        }

        Ok(())
    }
}
