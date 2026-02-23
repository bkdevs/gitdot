use crate::{
    config::Config,
    service::{Service, ServiceManager},
};

pub async fn start(config: Config) -> anyhow::Result<()> {
    let manager = ServiceManager::new(config.ci.run_as_user)?;
    manager.start()?;
    println!("Runner service started.");
    Ok(())
}
