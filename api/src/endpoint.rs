use serde::{Deserialize, Serialize};

pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: http::Method;

    type Request: Serialize + for<'de> Deserialize<'de> + Send;
    type Response: Serialize + for<'de> Deserialize<'de>;
}

pub mod dag;
pub mod organization;
pub mod question;
pub mod repository;
pub mod runner;
pub mod task;
pub mod user;

pub use dag::*;
pub use organization::*;
pub use question::*;
pub use repository::*;
pub use runner::*;
pub use task::*;
pub use user::*;
