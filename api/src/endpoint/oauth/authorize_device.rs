use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::endpoint::Endpoint;

pub struct AuthorizeDevice;

impl Endpoint for AuthorizeDevice {
    const PATH: &'static str = "/oauth/authorize";
    const METHOD: http::Method = http::Method::POST;

    type Request = AuthorizeDeviceRequest;
    type Response = ();
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct AuthorizeDeviceRequest {
    pub user_code: String,
}
