use serde::{Deserialize, Serialize};

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthTokensResource {
    pub access_token: String,
    pub refresh_token: String,
}
