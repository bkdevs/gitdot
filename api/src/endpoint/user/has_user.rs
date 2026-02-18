use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::endpoint::Endpoint;

pub struct HasUser;

impl Endpoint for HasUser {
    const PATH: &'static str = "/user/{user_name}";
    const METHOD: http::Method = http::Method::HEAD;

    type Request = HasUserApiRequest;
    type Response = HasUserResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct HasUserApiRequest {
    pub user_name: String,
}

pub type HasUserResponse = ();
