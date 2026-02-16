use crate::{
    command::{
        Args,
        auth::{AuthCommand, get_status, login},
    },
    config::Config,
};

pub async fn run(args: &Args) -> anyhow::Result<()> {
    let config = Config::load().await?;
    match &args {
        Args::Auth(auth_args) => match &auth_args.command {
            AuthCommand::Login => login(config).await?,
            AuthCommand::Status => get_status(config).await?,
        },
    }
    Ok(())
}
