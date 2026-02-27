use crate::config::runner::RUNNER_CONFIG_PATH;

pub async fn config() -> anyhow::Result<()> {
    let editor = std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string());

    println!("Editing {}", RUNNER_CONFIG_PATH);

    crate::util::run_command("sudo", &[&editor, RUNNER_CONFIG_PATH])
}
