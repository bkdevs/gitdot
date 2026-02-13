use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct HasUser;

impl Endpoint for HasUser {
    const PATH: &'static str = "/user/{user_name}";
    const METHOD: http::Method = http::Method::HEAD;

    type ApiRequest = HasUserApiRequest;
    type ApiResponse = HasUserApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasUserApiRequest {
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasUserApiResponse {}
