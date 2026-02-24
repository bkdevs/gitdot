const CONFIG_FILENAME: &str = ".gitdot-ci.toml";

const TEMPLATE: &str = r#"[pull_request]
tasks = ["build", "lint", "test"]

[[tasks]]
name = "build"
script = "echo 'Your build command here'"

[[tasks]]
name = "lint"
script = "echo 'Your lint command here'"

[[tasks]]
name = "test"
script = "echo 'Your test command here'"
runs_after = ["build", "lint"]
"#;

pub async fn init() -> anyhow::Result<()> {
    let output = tokio::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .await?;

    if !output.status.success() {
        anyhow::bail!("Not inside a git repository");
    }

    let git_root = std::path::PathBuf::from(String::from_utf8(output.stdout)?.trim());
    let config_path = git_root.join(CONFIG_FILENAME);

    if config_path.exists() {
        anyhow::bail!("{} already exists", config_path.display());
    }

    tokio::fs::write(&config_path, TEMPLATE).await?;
    println!("Created {}", config_path.display());

    Ok(())
}
