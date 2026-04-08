use gitdot_api::{
    endpoint::user::update_current_user_settings as api_endpoint, resource::user as api,
};
use gitdot_core::{
    dto::{UserRepoSettingsResponse, UserResponse, UserSettingsResponse},
    model::UserRepoSettings,
};

use super::{FromApi, IntoApi};

impl FromApi for UserRepoSettings {
    type ApiType = api_endpoint::UpdateUserRepoSettingsRequest;
    fn from_api(r: Self::ApiType) -> Self {
        UserRepoSettings {
            commit_filters: <Option<Vec<_>>>::from_api(r.commit_filters),
        }
    }
}

impl IntoApi for UserRepoSettingsResponse {
    type ApiType = api::UserRepoSettingsResource;
    fn into_api(self) -> Self::ApiType {
        api::UserRepoSettingsResource {
            commit_filters: self.commit_filters.into_api(),
        }
    }
}

impl IntoApi for UserSettingsResponse {
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
            location: self.location,
            readme: self.readme,
            website: self.website,
            company: self.company,
        }
    }
}
