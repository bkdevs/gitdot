use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::AuthorizationError;

#[derive(Debug, Clone)]
pub enum RepositoryIdentifier {
    Id(Uuid),
    OwnerRepo {
        owner_name: OwnerName,
        repo_name: RepositoryName,
    },
}

#[derive(Debug, Clone)]
pub struct RepositoryAuthorizationRequest {
    pub user_id: Option<Uuid>,
    pub repository: RepositoryIdentifier,
}

impl RepositoryAuthorizationRequest {
    pub fn with_id(user_id: Option<Uuid>, repository_id: Uuid) -> Self {
        Self {
            user_id,
            repository: RepositoryIdentifier::Id(repository_id),
        }
    }

    pub fn with_owner_repo(
        user_id: Option<Uuid>,
        owner_name: &str,
        repo_name: &str,
    ) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id,
            repository: RepositoryIdentifier::OwnerRepo {
                owner_name: OwnerName::try_new(owner_name)
                    .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
                repo_name: RepositoryName::try_new(repo_name)
                    .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            },
        })
    }
}
