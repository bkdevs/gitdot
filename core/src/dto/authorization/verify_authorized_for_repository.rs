use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::AuthorizationError;

#[derive(Debug, Clone)]
pub struct RepositoryAuthorizationRequest {
    pub user_id: Option<Uuid>,
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl RepositoryAuthorizationRequest {
    pub fn new(user_id: Option<Uuid>, owner: &str, repo: &str) -> Result<Self, AuthorizationError> {
        Ok(Self {
            user_id: user_id,
            owner: OwnerName::try_new(owner)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| AuthorizationError::InvalidRequest(e.to_string()))?,
        })
    }

    pub fn get_repo_path(&self) -> String {
        format!("{}/{}", self.owner.as_ref(), self.repo.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request_with_user() {
        let user_id = Uuid::new_v4();
        let request =
            RepositoryAuthorizationRequest::new(Some(user_id), "johndoe", "my-repo").unwrap();

        assert_eq!(request.user_id, Some(user_id));
        assert_eq!(request.owner.as_ref(), "johndoe");
        assert_eq!(request.repo.as_ref(), "my-repo");
    }

    #[test]
    fn valid_request_without_user() {
        let request = RepositoryAuthorizationRequest::new(None, "johndoe", "my-repo").unwrap();

        assert_eq!(request.user_id, None);
        assert_eq!(request.owner.as_ref(), "johndoe");
    }

    #[test]
    fn get_repo_path_formats_correctly() {
        let request = RepositoryAuthorizationRequest::new(None, "johndoe", "my-repo").unwrap();

        assert_eq!(request.get_repo_path(), "johndoe/my-repo");
    }

    #[test]
    fn sanitizes_to_lowercase() {
        let request = RepositoryAuthorizationRequest::new(None, "JohnDoe", "MyRepo").unwrap();

        assert_eq!(request.get_repo_path(), "johndoe/myrepo");
    }

    #[test]
    fn rejects_invalid_owner() {
        let result = RepositoryAuthorizationRequest::new(None, "invalid@owner", "my-repo");

        assert!(matches!(result, Err(AuthorizationError::InvalidRequest(_))));
    }

    #[test]
    fn rejects_invalid_repo() {
        let result = RepositoryAuthorizationRequest::new(None, "johndoe", "invalid/repo");

        assert!(matches!(result, Err(AuthorizationError::InvalidRequest(_))));
    }
}
