use crate::util::ci;

const TEMPLATE: &str = r#"#:schema https://www.gitdot.io/schema/gitdot-ci.json
[pull_request]
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
    let config_path = ci::find_config().await?;

    if config_path.exists() {
        anyhow::bail!("{} already exists", config_path.display());
    }

    tokio::fs::write(&config_path, TEMPLATE).await?;
    println!("Created {}", config_path.display());

    Ok(())
}
