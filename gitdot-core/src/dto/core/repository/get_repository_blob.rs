use crate::{
    dto::{OwnerName, RepositoryFileResponse, RepositoryName, RepositoryPath},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct GetRepositoryBlobRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
    pub path: String,
}

impl GetRepositoryBlobRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
        path: String,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| InputError::new("repository name", e))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| InputError::new("owner name", e))?,
            ref_name,
            path,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryFolderResponse {
    pub commit_sha: String,
    pub path: String,
    pub entries: Vec<RepositoryPath>,
}

#[derive(Debug, Clone)]
pub enum RepositoryBlobResponse {
    File(RepositoryFileResponse),
    Folder(RepositoryFolderResponse),
}
