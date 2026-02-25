use gitdot_api::resource::migration as api;
use gitdot_core::{
    dto::{
        GitHubInstallationResponse, GitHubRepositoryResponse, MigrationRepositoryResponse,
        MigrationResponse,
    },
    model::{
        GitHubInstallationType, MigrationOriginService, MigrationRepositoryStatus, MigrationStatus,
    },
};

use super::IntoApi;

impl IntoApi for GitHubInstallationResponse {
    type ApiType = api::GitHubInstallationResource;
    fn into_api(self) -> Self::ApiType {
        api::GitHubInstallationResource {
            id: self.id,
            installation_id: self.installation_id,
            owner_id: self.owner_id,
            installation_type: match self.installation_type {
                GitHubInstallationType::User => "user".to_string(),
                GitHubInstallationType::Organization => "organization".to_string(),
            },
            github_login: self.github_login,
            created_at: self.created_at,
        }
    }
}

impl IntoApi for GitHubRepositoryResponse {
    type ApiType = api::GitHubRepositoryResource;
    fn into_api(self) -> Self::ApiType {
        api::GitHubRepositoryResource {
            id: self.id,
            name: self.name,
            full_name: self.full_name,
            description: self.description,
            private: self.private,
            default_branch: self.default_branch,
        }
    }
}

impl IntoApi for MigrationResponse {
    type ApiType = api::MigrationResource;
    fn into_api(self) -> Self::ApiType {
        api::MigrationResource {
            id: self.id,
            number: self.number,
            author_id: self.author_id,
            origin_service: match self.origin_service {
                MigrationOriginService::GitHub => "github".to_string(),
            },
            origin: self.origin,
            origin_type: self.origin_type.into(),
            destination: self.destination,
            destination_type: self.destination_type.into(),
            status: match self.status {
                MigrationStatus::Pending => "pending".to_string(),
                MigrationStatus::Running => "running".to_string(),
                MigrationStatus::Completed => "completed".to_string(),
                MigrationStatus::Failed => "failed".to_string(),
            },
            repositories: self
                .repositories
                .into_iter()
                .map(|r| r.into_api())
                .collect(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl IntoApi for MigrationRepositoryResponse {
    type ApiType = api::MigrationRepositoryResource;
    fn into_api(self) -> Self::ApiType {
        api::MigrationRepositoryResource {
            id: self.id,
            origin_full_name: self.origin_full_name,
            destination_full_name: self.destination_full_name,
            status: match self.status {
                MigrationRepositoryStatus::Pending => "pending".to_string(),
                MigrationRepositoryStatus::Running => "running".to_string(),
                MigrationRepositoryStatus::Completed => "completed".to_string(),
                MigrationRepositoryStatus::Failed => "failed".to_string(),
            },
            error: self.error,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
