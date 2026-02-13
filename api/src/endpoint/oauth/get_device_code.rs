use crate::endpoint::Endpoint;
use crate::resource::oauth::DeviceCodeResource;
use serde::{Deserialize, Serialize};

pub struct GetDeviceCode;

impl Endpoint for GetDeviceCode {
    const PATH: &'static str = "/oauth/device";
    const METHOD: http::Method = http::Method::POST;

    type Request = GetDeviceCodeRequest;
    type Response = GetDeviceCodeResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeviceCodeRequest {
    pub client_id: String,
}

pub type GetDeviceCodeResponse = DeviceCodeResource;
