use serde::{Deserialize, Serialize};

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthTokensResource {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHubAuthRedirectResource {
    pub authorize_url: String,
    pub state: String,
}
