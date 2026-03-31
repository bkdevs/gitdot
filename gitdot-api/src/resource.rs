pub mod auth;
pub mod build;
pub mod migration;
pub mod oauth;
pub mod organization;
pub mod question;
pub mod repository;
pub mod review;
pub mod runner;
pub mod settings;
pub mod task;
pub mod user;
pub mod webhook;

use serde::{Serialize, de::DeserializeOwned};

pub use auth::*;
pub use build::*;
pub use migration::*;
pub use oauth::*;
pub use organization::*;
pub use question::*;
pub use repository::*;
pub use review::*;
pub use runner::*;
pub use settings::*;
pub use task::*;
pub use user::*;
pub use webhook::*;

pub trait ApiResource: Serialize + PartialEq + DeserializeOwned {}

impl<T: ApiResource> ApiResource for Vec<T> {}

impl<T: ApiResource> ApiResource for Option<T> {}

impl ApiResource for () {}
