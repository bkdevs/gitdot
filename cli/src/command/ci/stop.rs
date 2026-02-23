use crate::os::{Service, ServiceManager};

pub async fn stop() -> anyhow::Result<()> {
    let manager = ServiceManager::new()?;
    manager.stop()?;
    println!("Runner service stopped.");
    Ok(())
}
