use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct ResendVerificationCode;

impl Endpoint for ResendVerificationCode {
    const PATH: &'static str = "/auth/account/resend-code";
    const METHOD: http::Method = http::Method::POST;

    type Request = ResendVerificationCodeRequest;
    type Response = ();
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ResendVerificationCodeRequest {
    pub email: String,
}
