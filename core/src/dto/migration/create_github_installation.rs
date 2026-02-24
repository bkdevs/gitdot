use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{GitHubInstallation, GitHubInstallationType};

#[derive(Debug, Clone)]
pub struct CreateGitHubInstallationRequest {
    pub installation_id: i64,
    pub owner_id: Uuid,
}

impl CreateGitHubInstallationRequest {
    pub fn new(installation_id: i64, owner_id: Uuid) -> Self {
        Self {
            installation_id,
            owner_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GitHubInstallationResponse {
    pub id: Uuid,
    pub installation_id: i64,
    pub owner_id: Uuid,
    pub installation_type: GitHubInstallationType,
    pub created_at: DateTime<Utc>,
}

impl From<GitHubInstallation> for GitHubInstallationResponse {
    fn from(i: GitHubInstallation) -> Self {
        Self {
            id: i.id,
            installation_id: i.installation_id,
            owner_id: i.owner_id,
            installation_type: i.r#type,
            created_at: i.created_at,
        }
    }
}
