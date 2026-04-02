use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::auth::TokenResource};

pub struct PollToken;

impl Endpoint for PollToken {
    const PATH: &'static str = "/auth/device/token";
    const METHOD: http::Method = http::Method::POST;

    type Request = PollTokenRequest;
    type Response = PollTokenResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct PollTokenRequest {
    pub device_code: String,
    pub client_id: String,
}

pub type PollTokenResponse = TokenResource;
