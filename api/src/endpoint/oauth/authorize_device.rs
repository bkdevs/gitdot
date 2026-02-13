use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct AuthorizeDevice;

impl Endpoint for AuthorizeDevice {
    const PATH: &'static str = "/oauth/authorize";
    const METHOD: http::Method = http::Method::POST;

    type Request = AuthorizeDeviceRequest;
    type Response = ();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizeDeviceRequest {
    pub user_code: String,
}
