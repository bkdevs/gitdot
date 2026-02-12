use crate::{
    EndpointResponse,
    endpoint::{Endpoint, EndpointRequest, Method},
};
use serde::{Deserialize, Serialize};

struct HasUser;

impl Endpoint for HasUser {
    type Request = HasUserEndpointRequest;
    type Response = HasUserEndpointResponse;

    const METHOD: Method = Method::HEAD;
}

#[derive(Debug, Serialize, Deserialize)]
struct HasUserEndpointRequest {
    pub user_name: String,
}

impl EndpointRequest for HasUserEndpointRequest {
    fn url(&self) -> String {
        format!("/user/{}", self.user_name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HasUserEndpointResponse {}

impl EndpointResponse for HasUserEndpointResponse {}
