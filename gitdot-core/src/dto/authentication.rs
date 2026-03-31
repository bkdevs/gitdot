mod issue_task_jwt;
mod logout;
mod refresh_session;
mod send_auth_email;
mod verify_auth_code;

use serde::{Deserialize, Deserializer, Serialize};

pub use issue_task_jwt::{IssueTaskJwtRequest, IssueTaskJwtResponse};
pub use logout::LogoutRequest;
pub use refresh_session::RefreshSessionRequest;
pub use send_auth_email::SendAuthEmailRequest;
pub use verify_auth_code::VerifyAuthCodeRequest;

#[derive(Debug, Clone)]
pub struct AuthTokensResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_in: u64,
    pub refresh_token_expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iss: String,
    #[serde(deserialize_with = "deserialize_aud")]
    pub aud: Vec<String>,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitdotClaims {
    pub iss: String,
    pub aud: Vec<String>,
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub user_metadata: UserMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMetadata {
    pub username: String,
    pub orgs: Vec<String>,
}

fn deserialize_aud<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    Ok(match OneOrMany::deserialize(d)? {
        OneOrMany::One(s) => vec![s],
        OneOrMany::Many(v) => v,
    })
}
