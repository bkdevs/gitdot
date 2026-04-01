use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OAuthRedirectResponse {
    pub authorize_url: String,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthStatePayload {
    pub nonce: String,
    pub exp: u64,
}
