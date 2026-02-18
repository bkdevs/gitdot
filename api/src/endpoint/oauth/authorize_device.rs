use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct AuthorizeDevice;

impl Endpoint for AuthorizeDevice {
    const PATH: &'static str = "/oauth/authorize";
    const METHOD: http::Method = http::Method::POST;

    type Request = AuthorizeDeviceRequest;
    type Response = ();
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct AuthorizeDeviceRequest {
    pub user_code: String,
}
