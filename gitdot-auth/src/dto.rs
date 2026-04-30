use gitdot_api::resource::{
    auth::{AuthTokensResource, DeviceCodeResource, GitHubAuthRedirectResource, TokenResource},
    slack::SlackAccountResource,
};
use gitdot_core::dto::{
    AuthTokensResponse, DeviceCodeResponse, LinkSlackAccountResponse, OAuthRedirectResponse,
    TokenResponse,
};

pub trait IntoApi {
    type ApiType;
    fn into_api(self) -> Self::ApiType;
}

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

impl IntoApi for LinkSlackAccountResponse {
    type ApiType = SlackAccountResource;
    fn into_api(self) -> Self::ApiType {
        SlackAccountResource {
            id: self.id,
            gitdot_user_id: self.gitdot_user_id,
            slack_user_id: self.slack_user_id,
            slack_team_id: self.slack_team_id,
            created_at: self.created_at,
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
