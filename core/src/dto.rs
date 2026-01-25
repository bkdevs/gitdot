mod authorization;
mod git_http;
mod organization;
mod repository;
mod user;

pub(crate) mod common;

/// Define commonly used newtypes within the module
use common::{OwnerName, RepositoryName};

/// Re-export to expose flattened namespace to public
pub use authorization::RepositoryAuthorizationRequest;
pub use git_http::{GitHttpResponse, InfoRefsRequest, ReceivePackRequest, UploadPackRequest};
pub use organization::{CreateOrganizationRequest, GetOrganizationRequest, OrganizationResponse};
pub use repository::{
    CreateRepositoryRequest, GetRepositoryCommitsRequest, GetRepositoryFileCommitsRequest,
    GetRepositoryFileRequest, GetRepositoryTreeRequest, RepositoryCommitResponse,
    RepositoryCommitsResponse, RepositoryFileResponse, RepositoryResponse, RepositoryTreeEntry,
    RepositoryTreeResponse,
};
pub use user::{GetUserRequest, UserResponse};
