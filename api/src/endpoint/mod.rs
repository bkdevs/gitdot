use serde::{Deserialize, Serialize};

pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: http::Method;

    type Request: EndpointRequest;
    type Response: EndpointResponse;
}

pub trait EndpointRequest: Serialize + for<'de> Deserialize<'de> + Send {}

pub trait EndpointResponse: Serialize + for<'de> Deserialize<'de> {}

pub mod user;
