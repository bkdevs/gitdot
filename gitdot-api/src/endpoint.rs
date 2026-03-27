pub mod build;
pub mod migration;
pub mod oauth;
pub mod organization;
pub mod question;
pub mod repository;
pub mod review;
pub mod runner;
pub mod task;
pub mod user;
pub mod webhook;

use serde::{Serialize, de::DeserializeOwned};

use crate::ApiResource;

pub use build::*;
pub use migration::*;
pub use oauth::*;
pub use organization::*;
pub use question::*;
pub use repository::*;
pub use review::*;
pub use runner::*;
pub use task::*;
pub use user::*;
pub use webhook::*;

pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: http::Method;

    type Request: ApiRequest;
    type Response: ApiResource;
}

pub trait ApiRequest: Serialize + DeserializeOwned + Send {}

impl ApiRequest for () {}
