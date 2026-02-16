use crate::{
    dto::{OwnerName, RepositoryCommitResponse, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryTreeRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
}

impl GetRepositoryTreeRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            ref_name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryTreeResponse {
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryTreeEntry>,
}

#[derive(Debug, Clone)]
pub struct RepositoryTreeEntry {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
    pub commit: RepositoryCommitResponse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request() {
        let request =
            GetRepositoryTreeRequest::new("my-repo", "johndoe", "main".to_string()).unwrap();

        assert_eq!(request.name.as_ref(), "my-repo");
        assert_eq!(request.owner_name.as_ref(), "johndoe");
        assert_eq!(request.ref_name, "main");
    }

    #[test]
    fn valid_with_commit_sha() {
        let request =
            GetRepositoryTreeRequest::new("repo", "owner", "abc123def456".to_string()).unwrap();

        assert_eq!(request.ref_name, "abc123def456");
    }

    #[test]
    fn strips_git_suffix() {
        let request =
            GetRepositoryTreeRequest::new("my-repo.git", "johndoe", "main".to_string()).unwrap();

        assert_eq!(request.name.as_ref(), "my-repo");
    }

    #[test]
    fn rejects_invalid_repo_name() {
        let result = GetRepositoryTreeRequest::new("invalid/repo", "johndoe", "main".to_string());

        assert!(matches!(
            result,
            Err(RepositoryError::InvalidRepositoryName(_))
        ));
    }

    #[test]
    fn rejects_invalid_owner_name() {
        let result = GetRepositoryTreeRequest::new("my-repo", "invalid@owner", "main".to_string());

        assert!(matches!(result, Err(RepositoryError::InvalidOwnerName(_))));
    }
}
