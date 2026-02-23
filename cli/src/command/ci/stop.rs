use crate::{config::Config, os::{Service, ServiceManager}};

pub async fn stop(_config: Config) -> anyhow::Result<()> {
    let manager = ServiceManager::new()?;
    manager.stop()?;
    println!("Runner service stopped.");
    Ok(())
}
