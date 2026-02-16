use crate::{command::Args, config::Config};

pub async fn run(args: &Args) -> anyhow::Result<()> {
    let config = Config::load().await?;
    if let Some(command) = &args.command {
        command.execute(config).await?;
    }

    Ok(())
}
