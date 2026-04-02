use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::auth::DeviceCodeResource};

pub struct CreateDeviceCode;

impl Endpoint for CreateDeviceCode {
    const PATH: &'static str = "/auth/device/code";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateDeviceCodeRequest;
    type Response = CreateDeviceCodeResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateDeviceCodeRequest {
    pub client_id: String,
}

pub type CreateDeviceCodeResponse = DeviceCodeResource;
