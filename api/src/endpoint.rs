use serde::{Deserialize, Serialize};

pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: http::Method;

    type Request: Serialize + for<'de> Deserialize<'de> + Send;
    type Response: Serialize + for<'de> Deserialize<'de>;
}

pub mod repository;
pub mod user;

pub use repository::*;
pub use user::*;
