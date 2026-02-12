use super::UserEndpointResponse;
use crate::endpoint::{Endpoint, EndpointRequest, Method};
use serde::{Deserialize, Serialize};

struct GetUser;

impl Endpoint for GetUser {
    type Request = GetUserEndpointRequest;
    type Response = UserEndpointResponse;

    const METHOD: Method = Method::GET;
}

#[derive(Debug, Serialize, Deserialize)]
struct GetUserEndpointRequest {
    pub user_name: String,
}

impl EndpointRequest for GetUserEndpointRequest {
    fn url(&self) -> String {
        format!("/user/{}", self.user_name)
    }
}
