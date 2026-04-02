use gitdot_api::resource::auth::{
    AuthTokensResource, DeviceCodeResource, GitHubAuthRedirectResource, TokenResource,
};
use gitdot_core::dto::{
    AuthTokensResponse, DeviceCodeResponse, OAuthRedirectResponse, TokenResponse,
};

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

impl IntoApi for DeviceCodeResponse {
    type ApiType = DeviceCodeResource;
    fn into_api(self) -> Self::ApiType {
        DeviceCodeResource {
            device_code: self.device_code,
            user_code: self.user_code,
            verification_uri: self.verification_uri,
            expires_in: self.expires_in,
            interval: self.interval,
        }
    }
}

impl IntoApi for TokenResponse {
    type ApiType = TokenResource;
    fn into_api(self) -> Self::ApiType {
        TokenResource {
            access_token: self.access_token,
            user_name: self.user_name,
            user_email: self.user_email,
        }
    }
}
