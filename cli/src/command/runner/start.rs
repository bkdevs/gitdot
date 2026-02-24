use crate::os::{Service, ServiceManager};

pub async fn start() -> anyhow::Result<()> {
    let manager = ServiceManager::new()?;
    manager.start()?;
    println!("Runner service started.");
    Ok(())
}
