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
