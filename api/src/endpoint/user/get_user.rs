use super::UserApiResponse;
use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct GetUser;

impl Endpoint for GetUser {
    const PATH: &'static str = "/user/{user_name}";
    const METHOD: http::Method = http::Method::GET;

    type ApiRequest = GetUserApiRequest;
    type ApiResponse = UserApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserApiRequest {}
