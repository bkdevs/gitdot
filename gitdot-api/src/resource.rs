pub mod build;
pub mod migration;
pub mod oauth;
pub mod organization;
pub mod question;
pub mod repository;
pub mod runner;
pub mod task;
pub mod user;

use serde::{Serialize, de::DeserializeOwned};

pub use build::*;
pub use migration::*;
pub use oauth::*;
pub use organization::*;
pub use question::*;
pub use repository::*;
pub use runner::*;
pub use task::*;
pub use user::*;

pub trait ApiResource: Serialize + PartialEq + DeserializeOwned {}

impl<T: ApiResource> ApiResource for Vec<T> {}

impl<T: ApiResource> ApiResource for Option<T> {}

impl ApiResource for () {}
