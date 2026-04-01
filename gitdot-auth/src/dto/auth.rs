use gitdot_api::resource::auth::{AuthTokensResource, GitHubAuthRedirectResource};
use gitdot_core::dto::{AuthTokensResponse, OAuthRedirectResponse};

use super::IntoApi;

impl IntoApi for AuthTokensResponse {
    type ApiType = AuthTokensResource;
    fn into_api(self) -> Self::ApiType {
        AuthTokensResource {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            access_token_expires_in: self.access_token_expires_in,
            refresh_token_expires_in: self.refresh_token_expires_in,
            is_new: self.is_new,
        }
    }
}

impl IntoApi for OAuthRedirectResponse {
    type ApiType = GitHubAuthRedirectResource;
    fn into_api(self) -> Self::ApiType {
        GitHubAuthRedirectResource {
            authorize_url: self.authorize_url,
            state: self.state,
        }
    }
}
