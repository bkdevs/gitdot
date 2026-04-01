use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct Logout;

impl Endpoint for Logout {
    const PATH: &'static str = "/auth/logout";
    const METHOD: http::Method = http::Method::POST;

    type Request = LogoutRequest;
    type Response = ();
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}
