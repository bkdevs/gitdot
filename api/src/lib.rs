#[macro_use]
extern crate gitdot_api_derive;

pub mod endpoint;
pub mod resource;

pub use endpoint::ApiRequest;
pub use resource::ApiResource;
