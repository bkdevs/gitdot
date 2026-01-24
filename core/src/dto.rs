mod common;
mod git_http;
mod organization;
mod repository;
mod user;

/// Define commonly used newtypes within the module
use common::{OwnerName, RepositoryName};

/// Re-export to expose flattened namespace to public
pub use git_http::{GitHttpResponse, InfoRefsRequest, ReceivePackRequest, UploadPackRequest};
pub use organization::{CreateOrganizationRequest, GetOrganizationRequest, OrganizationResponse};
pub use repository::{CreateRepositoryRequest, RepositoryResponse};
pub use user::{GetUserRequest, UserResponse};
