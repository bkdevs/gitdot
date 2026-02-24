const CONFIG_FILENAME: &str = ".gitdot-ci.toml";

/// Returns the path to .gitdot-ci.toml in the git repo root.
/// Does not check whether the file exists.
pub async fn find_config() -> anyhow::Result<std::path::PathBuf> {
    let output = tokio::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .await?;

    if !output.status.success() {
        anyhow::bail!("Not inside a git repository");
    }

    let git_root = std::path::PathBuf::from(String::from_utf8(output.stdout)?.trim());
    Ok(git_root.join(CONFIG_FILENAME))
}
