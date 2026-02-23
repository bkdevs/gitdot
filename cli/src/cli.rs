use crate::{command::Args, config::Config};

pub async fn run(args: &Args) -> anyhow::Result<()> {
    let config = Config::load().await?;
    match args {
        Args::Auth(auth_args) => auth_args.command.execute(config).await,
        Args::Ci(ci_args) => ci_args.command.execute(config).await,
    }
}
