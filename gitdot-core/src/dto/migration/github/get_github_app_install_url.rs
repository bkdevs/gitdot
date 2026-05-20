use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::GitHubAppInstallAction;
use crate::error::MigrationError;

#[derive(Debug, Clone)]
pub struct GetGitHubAppInstallUrlRequest {
    pub owner_id: Uuid,
    pub action: GitHubAppInstallAction,
}

impl GetGitHubAppInstallUrlRequest {
    pub fn new(owner_id: Uuid, action: &str) -> Result<Self, MigrationError> {
        Ok(Self {
            owner_id,
            action: GitHubAppInstallAction::try_from_str(action)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetGitHubAppInstallUrlResponse {
    pub install_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallStatePayload {
    pub user_id: Uuid,
    pub action: GitHubAppInstallAction,
    pub exp: i64,
}
