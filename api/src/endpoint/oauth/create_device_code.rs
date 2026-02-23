use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::oauth::DeviceCodeResource};

pub struct CreateDeviceCode;

impl Endpoint for CreateDeviceCode {
    const PATH: &'static str = "/oauth/device";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateDeviceCodeRequest;
    type Response = CreateDeviceCodeResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateDeviceCodeRequest {
    pub client_id: String,
}

pub type CreateDeviceCodeResponse = DeviceCodeResource;
