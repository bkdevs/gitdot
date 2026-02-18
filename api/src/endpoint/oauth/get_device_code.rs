use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::oauth::DeviceCodeResource};

pub struct GetDeviceCode;

impl Endpoint for GetDeviceCode {
    const PATH: &'static str = "/oauth/device";
    const METHOD: http::Method = http::Method::POST;

    type Request = GetDeviceCodeRequest;
    type Response = GetDeviceCodeResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetDeviceCodeRequest {
    pub client_id: String,
}

pub type GetDeviceCodeResponse = DeviceCodeResource;
