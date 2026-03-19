use gitdot_api::resource::user as api;
use gitdot_core::{
    dto::UserResponse,
    model::{UserRepoSettings, UserSettings},
};

use super::IntoApi;

impl IntoApi for UserRepoSettings {
    type ApiType = api::UserRepoSettingsResource;
    fn into_api(self) -> Self::ApiType {
        api::UserRepoSettingsResource {
            commit_filters: self.commit_filters.into_api(),
        }
    }
}

impl IntoApi for UserSettings {
    type ApiType = api::UserSettingsResource;
    fn into_api(self) -> Self::ApiType {
        api::UserSettingsResource {
            repos: self
                .repos
                .into_iter()
                .map(|(k, v)| (k, v.into_api()))
                .collect(),
        }
    }
}

impl IntoApi for UserResponse {
    type ApiType = api::UserResource;
    fn into_api(self) -> Self::ApiType {
        api::UserResource {
            id: self.id,
            name: self.name,
            email: self.email,
            created_at: self.created_at,
            settings: self.settings.into_api(),
        }
    }
}
