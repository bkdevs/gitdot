use crate::{
    dto::{OwnerName, RepositoryName},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct GetRepositoryPathsRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
}

impl GetRepositoryPathsRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| InputError::new("repository name", e))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| InputError::new("owner name", e))?,
            ref_name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryPathsResponse {
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryPath>,
}

#[derive(Debug, Clone)]
pub struct RepositoryPath {
    pub path: String,
    pub name: String,
    pub path_type: PathType,
    pub sha: String,
}

#[derive(Debug, Clone)]
pub enum PathType {
    Blob,
    Tree,
    Commit,
    Unknown,
}

impl PathType {
    pub fn from_git2(kind: Option<git2::ObjectType>) -> Self {
        match kind {
            Some(git2::ObjectType::Blob) => Self::Blob,
            Some(git2::ObjectType::Tree) => Self::Tree,
            Some(git2::ObjectType::Commit) => Self::Commit,
            _ => Self::Unknown,
        }
    }
}
