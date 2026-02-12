use crate::command::{Args, Commands, register};
use crate::config::Config;

pub async fn run(args: &Args) -> anyhow::Result<()> {
    let config = Config::load().await?;
    match &args.command {
        Some(Commands::Register {}) => {
            register(config).await?;
        }
        Some(Commands::Run {}) => {
            println!("run");
        }
        Some(Commands::Install {}) => {
            println!("install");
        }
        Some(Commands::Start {}) => {
            println!("start");
        }
        Some(Commands::Stop {}) => {
            println!("stop");
        }
        None => {}
    }
    Ok(())
}
