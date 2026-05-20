use serde::Deserialize;

use super::GithubInstallation;
use crate::error::{InputError, WebhookError};

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessGithubInstallationRequest {
    pub action: String,
    pub installation: GithubInstallation,
}

impl ProcessGithubInstallationRequest {
    pub fn new(body: &[u8]) -> Result<Self, WebhookError> {
        serde_json::from_slice(body)
            .map_err(|e| InputError::new("github installation body", e.to_string()).into())
    }
}
