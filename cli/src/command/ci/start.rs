use crate::{config::Config, os::{Service, ServiceManager}};

pub async fn start(_config: Config) -> anyhow::Result<()> {
    let manager = ServiceManager::new()?;
    manager.start()?;
    println!("Runner service started.");
    Ok(())
}
