use serde::{Deserialize, Serialize};

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthTokensResource {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_in: u64,
    pub refresh_token_expires_in: u64,
    pub is_new: bool,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHubAuthRedirectResource {
    pub authorize_url: String,
    pub state: String,
}
