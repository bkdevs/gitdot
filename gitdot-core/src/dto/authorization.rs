mod get_repository_permission;
mod validate_token;
mod verify_authorized_for_answer;
mod verify_authorized_for_comment;
mod verify_authorized_for_migration;
mod verify_authorized_for_organization;
mod verify_authorized_for_question;
mod verify_authorized_for_repository;
mod verify_authorized_for_repository_creation;

pub use get_repository_permission::{
    GetRepositoryPermissionRequest, GetRepositoryPermissionResponse,
};
pub use validate_token::{ValidateTokenRequest, ValidateTokenResponse};
pub use verify_authorized_for_answer::AnswerAuthorizationRequest;
pub use verify_authorized_for_comment::CommentAuthorizationRequest;
pub use verify_authorized_for_migration::MigrationAuthorizationRequest;
pub use verify_authorized_for_organization::OrganizationAuthorizationRequest;
pub use verify_authorized_for_question::QuestionAuthorizationRequest;
pub use verify_authorized_for_repository::RepositoryAuthorizationRequest;
pub use verify_authorized_for_repository_creation::RepositoryCreationAuthorizationRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryPermission {
    Read,
    Write,
    Admin,
}

impl Into<String> for RepositoryPermission {
    fn into(self) -> String {
        match self {
            RepositoryPermission::Read => "read".to_string(),
            RepositoryPermission::Write => "write".to_string(),
            RepositoryPermission::Admin => "admin".to_string(),
        }
    }
}
