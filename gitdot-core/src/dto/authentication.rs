use serde::{Deserialize, Serialize};

pub const GITDOT_SERVER_ID: &str = "gitdot-server";
pub const S2_SERVER_ID: &str = "s2-server";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub aud: Vec<String>,
    pub iss: String,
}

mod issue_task_token;
pub use issue_task_token::{IssueTaskTokenRequest, IssueTaskTokenResponse};
