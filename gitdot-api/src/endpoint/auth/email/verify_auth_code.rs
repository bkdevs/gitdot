use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::auth::AuthTokensResource};

pub struct VerifyAuthCode;

impl Endpoint for VerifyAuthCode {
    const PATH: &'static str = "/auth/email/verify";
    const METHOD: http::Method = http::Method::POST;

    type Request = VerifyAuthCodeRequest;
    type Response = AuthTokensResource;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct VerifyAuthCodeRequest {
    pub code: String,
}
