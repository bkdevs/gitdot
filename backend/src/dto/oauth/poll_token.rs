use serde::{Deserialize, Serialize};

use gitdot_core::dto::TokenResponse;

#[derive(Deserialize)]
pub struct PollTokenServerRequest {
    pub device_code: String,
    pub client_id: String,
}

#[derive(Serialize, PartialEq)]
pub struct TokenServerResponse {
    pub access_token: String,
    pub user_name: String,
    pub user_email: String,
}

impl From<TokenResponse> for TokenServerResponse {
    fn from(token: TokenResponse) -> Self {
        Self {
            access_token: token.access_token,
            user_name: token.user_name,
            user_email: token.user_email,
        }
    }
}
