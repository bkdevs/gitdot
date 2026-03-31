use gitdot_api::resource::auth::AuthTokensResource;
use gitdot_core::dto::AuthTokensResponse;

use super::IntoApi;

impl IntoApi for AuthTokensResponse {
    type ApiType = AuthTokensResource;
    fn into_api(self) -> Self::ApiType {
        AuthTokensResource {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
        }
    }
}
