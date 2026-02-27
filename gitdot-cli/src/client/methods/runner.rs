use anyhow::Result;

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn verify_runner(&self) -> Result<()> {
        self.post("ci/runner/verify".to_string(), ()).await
    }
}
