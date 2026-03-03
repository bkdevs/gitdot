use serde::{Deserialize, Serialize};

pub const GITDOT_SERVER_ID: &str = "gitdot-server";
pub const S2_SERVER_ID: &str = "s2-server";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iss: String,
    pub aud: Vec<String>,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

mod issue_internal_token;
pub use issue_internal_token::IssueInternalTokenResponse;

mod issue_task_token;
pub use issue_task_token::{IssueTaskTokenRequest, IssueTaskTokenResponse};
