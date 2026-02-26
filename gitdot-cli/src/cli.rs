use crate::command::Args;

pub async fn run(args: &Args) -> anyhow::Result<()> {
    match args {
        #[cfg(feature = "main")]
        Args::Auth(auth_args) => {
            let config = crate::config::UserConfig::load().await?;
            auth_args.command.execute(config).await
        }
        #[cfg(feature = "main")]
        Args::Ci(ci_args) => ci_args.command.execute().await,
        #[cfg(feature = "runner")]
        Args::Runner(runner_args) => {
            let config = crate::config::RunnerConfig::load()?;
            runner_args.command.execute(config).await
        }
    }
}
