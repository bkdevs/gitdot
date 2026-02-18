use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct HasUser;

impl Endpoint for HasUser {
    const PATH: &'static str = "/user/{user_name}";
    const METHOD: http::Method = http::Method::HEAD;

    type Request = HasUserApiRequest;
    type Response = HasUserResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct HasUserApiRequest {
    pub user_name: String,
}

pub type HasUserResponse = ();
