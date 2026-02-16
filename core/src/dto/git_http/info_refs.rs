use crate::{
    dto::{GitService, OwnerName, RepositoryName},
    error::GitHttpError,
};

#[derive(Debug, Clone)]
pub struct InfoRefsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub service: GitService,
}

impl InfoRefsRequest {
    pub fn new(owner: &str, repo: &str, service: &str) -> Result<Self, GitHttpError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| GitHttpError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| GitHttpError::InvalidRepositoryName(e.to_string()))?,
            service: GitService::try_new(service.to_string())
                .map_err(|e| GitHttpError::InvalidService(e.to_string()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod info_refs_request {
        use super::*;

        #[test]
        fn valid_upload_pack_request() {
            let request = InfoRefsRequest::new("johndoe", "my-repo", "git-upload-pack").unwrap();

            assert_eq!(request.owner.as_ref(), "johndoe");
            assert_eq!(request.repo.as_ref(), "my-repo");
            assert_eq!(request.service.as_ref(), "git-upload-pack");
        }

        #[test]
        fn valid_receive_pack_request() {
            let request = InfoRefsRequest::new("johndoe", "my-repo", "git-receive-pack").unwrap();

            assert_eq!(request.service.as_ref(), "git-receive-pack");
        }

        #[test]
        fn sanitizes_owner_and_repo() {
            let request = InfoRefsRequest::new("JohnDoe", "MyRepo.git", "git-upload-pack").unwrap();

            assert_eq!(request.owner.as_ref(), "johndoe");
            assert_eq!(request.repo.as_ref(), "myrepo");
        }

        #[test]
        fn rejects_invalid_owner() {
            let result = InfoRefsRequest::new("invalid@owner", "my-repo", "git-upload-pack");

            assert!(matches!(result, Err(GitHttpError::InvalidOwnerName(_))));
        }

        #[test]
        fn rejects_invalid_repo() {
            let result = InfoRefsRequest::new("johndoe", "invalid/repo", "git-upload-pack");

            assert!(matches!(
                result,
                Err(GitHttpError::InvalidRepositoryName(_))
            ));
        }

        #[test]
        fn rejects_invalid_service() {
            let result = InfoRefsRequest::new("johndoe", "my-repo", "invalid-service");

            assert!(matches!(result, Err(GitHttpError::InvalidService(_))));
        }

        #[test]
        fn rejects_empty_service() {
            let result = InfoRefsRequest::new("johndoe", "my-repo", "");

            assert!(matches!(result, Err(GitHttpError::InvalidService(_))));
        }
    }
}
