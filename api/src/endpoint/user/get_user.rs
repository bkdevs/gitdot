use super::UserEndpointResponse;
use crate::endpoint::{Endpoint, Method};
use serde::{Deserialize, Serialize};

struct GetUser;

impl Endpoint for GetUser {
    type Request = GetUserRequest;
    type Response = UserEndpointResponse;

    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/api/users/{id}";
}

#[derive(Debug, Serialize, Deserialize)]
struct GetUserRequest {
    pub user_id: u64,
}
