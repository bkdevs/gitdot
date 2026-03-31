mod issue_task_jwt;
mod send_auth_email;
mod verify_auth_code;

use serde::{Deserialize, Deserializer, Serialize};

pub use issue_task_jwt::{IssueTaskJwtRequest, IssueTaskJwtResponse};
pub use send_auth_email::SendAuthEmailRequest;
pub use verify_auth_code::{VerifyAuthCodeRequest, VerifyAuthCodeResponse};

pub const GITDOT_SERVER_ID: &str = "gitdot-server";
pub const S2_SERVER_ID: &str = "s2-server";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iss: String,
    #[serde(deserialize_with = "deserialize_aud")]
    pub aud: Vec<String>,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
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
