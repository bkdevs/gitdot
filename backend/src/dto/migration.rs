use gitdot_api::resource::migration as api;
use gitdot_core::{dto::GitHubInstallationResponse, model::GitHubInstallationType};

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
            created_at: self.created_at,
        }
    }
}
