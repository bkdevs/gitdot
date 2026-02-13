use serde::{Deserialize, Serialize};

pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: http::Method;

    type ApiRequest: Serialize + for<'de> Deserialize<'de> + Send;
    type ApiResponse: Serialize + for<'de> Deserialize<'de>;
}

pub mod repository;
pub mod user;
