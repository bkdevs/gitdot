use uuid::Uuid;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
    model::{RepositoryOwnerType, RepositoryVisibility},
};

#[derive(Debug, Clone)]
pub struct CreateRepositoryRequest {
    pub name: RepositoryName,
    pub user_id: Uuid,
    pub owner_name: OwnerName,
    pub owner_type: RepositoryOwnerType,
    pub visibility: RepositoryVisibility,
}

impl CreateRepositoryRequest {
    pub fn new(
        repo_name: &str,
        user_id: Uuid,
        owner_name: &str,
        owner_type: &str,
        visibility: &str,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            user_id,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            owner_type: owner_type.try_into()?,
            visibility: visibility.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request() {
        let user_id = Uuid::new_v4();
        let request =
            CreateRepositoryRequest::new("my-repo", user_id, "johndoe", "user", "public").unwrap();

        assert_eq!(request.name.as_ref(), "my-repo");
        assert_eq!(request.user_id, user_id);
        assert_eq!(request.owner_name.as_ref(), "johndoe");
        assert_eq!(request.owner_type, RepositoryOwnerType::User);
        assert_eq!(request.visibility, RepositoryVisibility::Public);
    }

    #[test]
    fn valid_private_org_repository() {
        let user_id = Uuid::new_v4();
        let request =
            CreateRepositoryRequest::new("repo", user_id, "myorg", "organization", "private")
                .unwrap();

        assert_eq!(request.owner_type, RepositoryOwnerType::Organization);
        assert_eq!(request.visibility, RepositoryVisibility::Private);
    }

    #[test]
    fn strips_git_suffix_from_repo_name() {
        let user_id = Uuid::new_v4();
        let request =
            CreateRepositoryRequest::new("my-repo.git", user_id, "johndoe", "user", "public")
                .unwrap();

        assert_eq!(request.name.as_ref(), "my-repo");
    }

    #[test]
    fn rejects_invalid_repo_name() {
        let user_id = Uuid::new_v4();
        let result =
            CreateRepositoryRequest::new("invalid/repo", user_id, "johndoe", "user", "public");

        assert!(matches!(
            result,
            Err(RepositoryError::InvalidRepositoryName(_))
        ));
    }

    #[test]
    fn rejects_invalid_owner_name() {
        let user_id = Uuid::new_v4();
        let result =
            CreateRepositoryRequest::new("my-repo", user_id, "invalid@owner", "user", "public");

        assert!(matches!(result, Err(RepositoryError::InvalidOwnerName(_))));
    }

    #[test]
    fn rejects_invalid_owner_type() {
        let user_id = Uuid::new_v4();
        let result =
            CreateRepositoryRequest::new("my-repo", user_id, "johndoe", "invalid", "public");

        assert!(matches!(result, Err(RepositoryError::InvalidOwnerType(_))));
    }

    #[test]
    fn rejects_invalid_visibility() {
        let user_id = Uuid::new_v4();
        let result = CreateRepositoryRequest::new("my-repo", user_id, "johndoe", "user", "invalid");

        assert!(matches!(result, Err(RepositoryError::InvalidVisibility(_))));
    }
}
