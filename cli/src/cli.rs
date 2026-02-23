use crate::command::Args;

pub async fn run(args: &Args) -> anyhow::Result<()> {
    match args {
        #[cfg(feature = "main")]
        Args::Auth(auth_args) => {
            let config = crate::config::UserConfig::load().await?;
            auth_args.command.execute(config).await
        }
        #[cfg(feature = "ci")]
        Args::Ci(ci_args) => {
            let config = crate::config::RunnerConfig::load()?;
            ci_args.command.execute(config).await
        }
    }
}
