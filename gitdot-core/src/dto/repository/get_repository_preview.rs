use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
    util::git::DEFAULT_PREVIEW_LINES,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryPreviewRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
    pub preview_lines: u32,
}

impl GetRepositoryPreviewRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
        preview_lines: Option<u32>,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            ref_name,
            preview_lines: preview_lines.unwrap_or(DEFAULT_PREVIEW_LINES),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryPreviewResponse {
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryPreviewEntry>,
}

#[derive(Debug, Clone)]
pub struct RepositoryPreviewEntry {
    pub path: String,
    pub name: String,
    pub sha: String,
    pub preview: Option<FilePreview>,
}

#[derive(Debug, Clone)]
pub struct FilePreview {
    pub content: String,
    pub total_lines: u32,
    pub preview_lines: u32,
    pub truncated: bool,
    pub encoding: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request() {
        let request =
            GetRepositoryPreviewRequest::new("my-repo", "johndoe", "main".to_string(), None)
                .unwrap();

        assert_eq!(request.name.as_ref(), "my-repo");
        assert_eq!(request.owner_name.as_ref(), "johndoe");
        assert_eq!(request.ref_name, "main");
        assert_eq!(request.preview_lines, DEFAULT_PREVIEW_LINES);
    }

    #[test]
    fn custom_preview_lines() {
        let request =
            GetRepositoryPreviewRequest::new("my-repo", "johndoe", "main".to_string(), Some(50))
                .unwrap();

        assert_eq!(request.preview_lines, 50);
    }

    #[test]
    fn sanitizes_inputs() {
        let request =
            GetRepositoryPreviewRequest::new("MyRepo.git", "JohnDoe", "main".to_string(), None)
                .unwrap();

        assert_eq!(request.name.as_ref(), "myrepo");
        assert_eq!(request.owner_name.as_ref(), "johndoe");
    }

    #[test]
    fn rejects_invalid_repo_name() {
        let result =
            GetRepositoryPreviewRequest::new("invalid/repo", "johndoe", "main".to_string(), None);

        assert!(matches!(
            result,
            Err(RepositoryError::InvalidRepositoryName(_))
        ));
    }

    #[test]
    fn rejects_invalid_owner_name() {
        let result =
            GetRepositoryPreviewRequest::new("my-repo", "invalid@owner", "main".to_string(), None);

        assert!(matches!(result, Err(RepositoryError::InvalidOwnerName(_))));
    }
}
