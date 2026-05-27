use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::auth::AuthTokensResource};

pub struct ExchangeGitHubCode;

impl Endpoint for ExchangeGitHubCode {
    const PATH: &'static str = "/auth/github/exchange";
    const METHOD: http::Method = http::Method::POST;

    type Request = ExchangeGitHubCodeRequest;
    type Response = AuthTokensResource;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ExchangeGitHubCodeRequest {
    pub code: String,
    pub state: String,
}
