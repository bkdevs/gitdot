use uuid::Uuid;

use super::{GitHubAppInstallAction, GitHubInstallationResponse};

#[derive(Debug, Clone)]
pub struct CreateGitHubInstallationRequest {
    pub installation_id: i64,
    pub owner_id: Uuid,
    pub state: String,
    pub code: String,
}

impl CreateGitHubInstallationRequest {
    pub fn new(installation_id: i64, owner_id: Uuid, state: String, code: String) -> Self {
        Self {
            installation_id,
            owner_id,
            state,
            code,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateGitHubInstallationResponse {
    pub installation: GitHubInstallationResponse,
    pub action: GitHubAppInstallAction,
}
