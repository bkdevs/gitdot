use super::UserApiResponse;
use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct GetCurrentUser;

impl Endpoint for GetCurrentUser {
    const PATH: &'static str = "/user";
    const METHOD: http::Method = http::Method::GET;

    type ApiRequest = GetCurrentUserApiRequest;
    type ApiResponse = UserApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCurrentUserApiRequest {}
