use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct VerifyAuthCode;

impl Endpoint for VerifyAuthCode {
    const PATH: &'static str = "/auth/email/verify";
    const METHOD: http::Method = http::Method::POST;

    type Request = VerifyAuthCodeRequest;
    type Response = VerifyAuthCodeResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct VerifyAuthCodeRequest {
    pub code: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyAuthCodeResponse {
    pub access_token: String,
    pub refresh_token: String,
}
