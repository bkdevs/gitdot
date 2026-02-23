use crate::{command::Args, config::Config};

pub async fn run(args: &Args) -> anyhow::Result<()> {
    let config = Config::load().await?;
    match args {
        #[cfg(feature = "main")]
        Args::Auth(auth_args) => auth_args.command.execute(config).await,
        #[cfg(feature = "ci")]
        Args::Ci(ci_args) => ci_args.command.execute(config).await,
    }
}
