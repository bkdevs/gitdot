use crate::util::ci;

const TEMPLATE: &str = r#"#:schema https://www.gitdot.io/schema/gitdot-ci.json

[[builds]]
trigger = "pull_request"
tasks = ["lint", "test", "build"]

[[builds]]
trigger = "push_to_main"
tasks = ["lint", "test", "build"]

[[tasks]]
name = "lint"
command = "echo 'Your lint command here'"

[[tasks]]
name = "test"
command = "echo 'Your test command here'"

[[tasks]]
name = "build"
command = "echo 'Your build command here'"
runs_after = ["lint", "test"]
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
