pub mod dag;
pub mod oauth;
pub mod organization;
pub mod question;
pub mod repository;
pub mod runner;
pub mod task;
pub mod user;

use serde::{Deserialize, Serialize};

use crate::ApiResource;

pub use dag::*;
pub use oauth::*;
pub use organization::*;
pub use question::*;
pub use repository::*;
pub use runner::*;
pub use task::*;
pub use user::*;

pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: http::Method;

    type Request: Serialize + for<'de> Deserialize<'de> + Send;
    type Response: ApiResource;
}
