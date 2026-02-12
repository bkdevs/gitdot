use serde::{Deserialize, Serialize};

pub trait Endpoint {
    type Request: EndpointRequest;
    type Response: EndpointResponse;

    const METHOD: Method;
}

pub trait EndpointRequest: Serialize + for<'de> Deserialize<'de> {
    fn url(&self) -> String;
}

pub trait EndpointResponse: Serialize + for<'de> Deserialize<'de> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::HEAD => "HEAD",
        }
    }
}

mod user;
