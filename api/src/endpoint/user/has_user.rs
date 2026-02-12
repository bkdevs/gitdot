use crate::endpoint::{Endpoint, EndpointRequest, EndpointResponse};
use serde::{Deserialize, Serialize};

struct HasUser;

impl Endpoint for HasUser {
    type Request = HasUserEndpointRequest;
    type Response = HasUserEndpointResponse;

    const PATH: &'static str = "/user/{user_name}";
    const METHOD: http::Method = http::Method::HEAD;
}

#[derive(Debug, Serialize, Deserialize)]
struct HasUserEndpointRequest {
    pub user_name: String,
}

impl EndpointRequest for HasUserEndpointRequest {}

#[derive(Debug, Serialize, Deserialize)]
struct HasUserEndpointResponse {}

impl EndpointResponse for HasUserEndpointResponse {}
