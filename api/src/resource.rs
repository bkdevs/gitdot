use serde::Serialize;

pub trait ApiResource: Serialize + PartialEq {}
impl<T: ApiResource> ApiResource for Vec<T> {}
impl<T: ApiResource> ApiResource for Option<T> {}
impl ApiResource for () {}

pub mod dag;
pub mod oauth;
pub mod organization;
pub mod question;
pub mod repository;
pub mod runner;
pub mod task;
pub mod user;

pub use dag::*;
pub use oauth::*;
pub use organization::*;
pub use question::*;
pub use repository::*;
pub use runner::*;
pub use task::*;
pub use user::*;
