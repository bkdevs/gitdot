use async_trait::async_trait;

use crate::dto::RepositoryAuthorizationRequest;
use crate::error::AuthorizationError;
use crate::repository::{
    OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
    RepositoryRepositoryImpl,
};

#[async_trait]
pub trait AuthorizationService: Send + Sync + 'static {
    async fn verify_authorized_for_repository(
        &self,
        request: RepositoryAuthorizationRequest,
    ) -> Result<(), AuthorizationError>;
}

#[derive(Debug, Clone)]
pub struct AuthorizationServiceImpl<O, R>
where
    O: OrganizationRepository,
    R: RepositoryRepository,
{
    org_repo: O,
    repo_repo: R,
}

impl AuthorizationServiceImpl<OrganizationRepositoryImpl, RepositoryRepositoryImpl> {
    pub fn new(org_repo: OrganizationRepositoryImpl, repo_repo: RepositoryRepositoryImpl) -> Self {
        Self {
            org_repo: org_repo,
            repo_repo: repo_repo,
        }
    }
}

#[async_trait]
impl<O, R> AuthorizationService for AuthorizationServiceImpl<O, R>
where
    O: OrganizationRepository,
    R: RepositoryRepository,
{
    async fn verify_authorized_for_repository(
        &self,
        request: RepositoryAuthorizationRequest,
    ) -> Result<(), AuthorizationError> {
        let repository = self
            .repo_repo
            .get(&request.owner_name, &request.repo_name)
            .await?
            .ok_or_else(|| {
                AuthorizationError::InvalidRequest(format!(
                    "Repository not found: {}",
                    &request.get_repo_path()
                ))
            })?;

        if repository.is_public() {
            return Ok(());
        }

        if request.user_id.is_none() {
            return Err(AuthorizationError::Unauthorized);
        }

        let user_id = request.user_id.unwrap();
        if repository.is_owned_by_user() {
            if repository.owner_id != user_id {
                return Err(AuthorizationError::Unauthorized);
            }
        } else {
            let is_member = self
                .org_repo
                .is_member(repository.owner_id, user_id)
                .await?;
            if !is_member {
                return Err(AuthorizationError::Unauthorized);
            }
        }

        Ok(())
    }
}
