use crate::{
    config::Config,
    service::{Service, ServiceManager},
};

pub async fn stop(config: Config) -> anyhow::Result<()> {
    let manager = ServiceManager::new(config.ci.run_as_user)?;
    manager.stop()?;
    println!("Runner service stopped.");
    Ok(())
}
