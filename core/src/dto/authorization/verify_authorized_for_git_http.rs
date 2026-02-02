use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::AuthorizationError;
use crate::model::GitOperation;

#[derive(Debug, Clone)]
pub struct GitHttpAuthorizationRequest {
    pub user_id: Option<Uuid>,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub operation: GitOperation,
}

impl GitHttpAuthorizationRequest {
    pub fn for_info_refs(
        user_id: Option<Uuid>,
        owner: &str,
        repo: &str,
        service: &str,
    ) -> Result<Self, AuthorizationError> {
        let operation = match service {
            "git-receive-pack" => GitOperation::Write,
            _ => GitOperation::Read,
        };
        Ok(Self {
            user_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            operation,
        })
    }

    pub fn for_upload_pack(
        user_id: Option<Uuid>,
        owner: &str,
        repo: &str,
    ) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            operation: GitOperation::Read,
        })
    }

    pub fn for_receive_pack(
        user_id: Option<Uuid>,
        owner: &str,
        repo: &str,
    ) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            operation: GitOperation::Write,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod for_info_refs {
        use super::*;

        #[test]
        fn read_operation_for_upload_pack_service() {
            let user_id = Uuid::new_v4();
            let request = GitHttpAuthorizationRequest::for_info_refs(
                Some(user_id),
                "johndoe",
                "my-repo",
                "git-upload-pack",
            )
            .unwrap();

            assert_eq!(request.user_id, Some(user_id));
            assert_eq!(request.owner.as_ref(), "johndoe");
            assert_eq!(request.repo.as_ref(), "my-repo");
            assert_eq!(request.operation, GitOperation::Read);
        }

        #[test]
        fn write_operation_for_receive_pack_service() {
            let request = GitHttpAuthorizationRequest::for_info_refs(
                None,
                "johndoe",
                "my-repo",
                "git-receive-pack",
            )
            .unwrap();

            assert_eq!(request.operation, GitOperation::Write);
        }

        #[test]
        fn defaults_to_read_for_unknown_service() {
            let request =
                GitHttpAuthorizationRequest::for_info_refs(None, "johndoe", "my-repo", "unknown")
                    .unwrap();

            assert_eq!(request.operation, GitOperation::Read);
        }

        #[test]
        fn rejects_invalid_owner() {
            let result = GitHttpAuthorizationRequest::for_info_refs(
                None,
                "invalid@owner",
                "my-repo",
                "git-upload-pack",
            );

            assert!(matches!(result, Err(AuthorizationError::InvalidRequest(_))));
        }

        #[test]
        fn rejects_invalid_repo() {
            let result = GitHttpAuthorizationRequest::for_info_refs(
                None,
                "johndoe",
                "invalid/repo",
                "git-upload-pack",
            );

            assert!(matches!(result, Err(AuthorizationError::InvalidRequest(_))));
        }
    }

    mod for_upload_pack {
        use super::*;

        #[test]
        fn creates_read_operation() {
            let user_id = Uuid::new_v4();
            let request =
                GitHttpAuthorizationRequest::for_upload_pack(Some(user_id), "johndoe", "my-repo")
                    .unwrap();

            assert_eq!(request.user_id, Some(user_id));
            assert_eq!(request.owner.as_ref(), "johndoe");
            assert_eq!(request.repo.as_ref(), "my-repo");
            assert_eq!(request.operation, GitOperation::Read);
        }

        #[test]
        fn works_without_user() {
            let request =
                GitHttpAuthorizationRequest::for_upload_pack(None, "johndoe", "my-repo").unwrap();

            assert_eq!(request.user_id, None);
            assert_eq!(request.operation, GitOperation::Read);
        }
    }

    mod for_receive_pack {
        use super::*;

        #[test]
        fn creates_write_operation() {
            let user_id = Uuid::new_v4();
            let request =
                GitHttpAuthorizationRequest::for_receive_pack(Some(user_id), "johndoe", "my-repo")
                    .unwrap();

            assert_eq!(request.user_id, Some(user_id));
            assert_eq!(request.owner.as_ref(), "johndoe");
            assert_eq!(request.repo.as_ref(), "my-repo");
            assert_eq!(request.operation, GitOperation::Write);
        }

        #[test]
        fn works_without_user() {
            let request =
                GitHttpAuthorizationRequest::for_receive_pack(None, "johndoe", "my-repo").unwrap();

            assert_eq!(request.user_id, None);
            assert_eq!(request.operation, GitOperation::Write);
        }
    }
}
