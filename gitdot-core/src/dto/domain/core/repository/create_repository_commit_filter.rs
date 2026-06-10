use uuid::Uuid;

use crate::{
    dto::{FilterName, OwnerName, RepositoryName, normalize_string_list},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct CreateRepositoryCommitFilterRequest {
    pub user_id: Uuid,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub name: FilterName,
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

impl CreateRepositoryCommitFilterRequest {
    pub fn new(
        user_id: Uuid,
        owner: &str,
        repo: &str,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            user_id,
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            name: FilterName::try_new(name).map_err(|e| InputError::new("filter name", e))?,
            authors: normalize_string_list(authors),
            tags: normalize_string_list(tags),
            paths: normalize_string_list(paths),
        })
    }
}
