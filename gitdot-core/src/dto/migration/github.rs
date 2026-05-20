mod create_github_installation;
mod create_github_migration;
mod get_github_app_install_url;
mod list_github_installation_repositories;
mod list_github_installations;
mod migrate_github_repositories;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{InputError, MigrationError},
    model::{GitHubInstallation, GitHubInstallationType},
};

pub use create_github_installation::{
    CreateGitHubInstallationRequest, CreateGitHubInstallationResponse,
};
pub use create_github_migration::{CreateGitHubMigrationRequest, CreateGitHubMigrationResponse};
pub use get_github_app_install_url::{
    GetGitHubAppInstallUrlRequest, GetGitHubAppInstallUrlResponse, InstallStatePayload,
};
pub use list_github_installation_repositories::{
    GitHubRepositoryResponse, ListGitHubInstallationRepositoriesRequest,
    ListGitHubInstallationRepositoriesResponse,
};
pub use list_github_installations::ListGitHubInstallationsRequest;
pub use migrate_github_repositories::{
    MigrateGitHubRepositoriesRequest, MigrateGitHubRepositoriesResponse, MigratedRepositoryInfo,
};

#[derive(Debug, Clone)]
pub struct GitHubInstallationResponse {
    pub id: Uuid,
    pub installation_id: i64,
    pub owner_id: Uuid,
    pub installation_type: GitHubInstallationType,
    pub github_login: String,
    pub created_at: DateTime<Utc>,
}

impl From<GitHubInstallation> for GitHubInstallationResponse {
    fn from(i: GitHubInstallation) -> Self {
        Self {
            id: i.id,
            installation_id: i.installation_id,
            owner_id: i.owner_id,
            installation_type: i.r#type,
            github_login: i.github_login,
            created_at: i.created_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GitHubAppInstallAction {
    Migration,
    Onboarding,
}

impl GitHubAppInstallAction {
    pub fn try_from_str(s: &str) -> Result<Self, MigrationError> {
        match s {
            "migration" => Ok(Self::Migration),
            "onboarding" => Ok(Self::Onboarding),
            other => Err(InputError::new("action", format!("invalid: {other}")).into()),
        }
    }
}
