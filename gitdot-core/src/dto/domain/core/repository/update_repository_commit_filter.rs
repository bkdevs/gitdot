use uuid::Uuid;

use crate::{
    dto::{FilterName, OwnerName, RepositoryName, normalize_string_list},
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct UpdateRepositoryCommitFilterRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub filter_id: Uuid,
    pub name: FilterName,
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

impl UpdateRepositoryCommitFilterRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        filter_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            filter_id,
            name: FilterName::try_new(name).map_err(|e| InputError::new("filter name", e))?,
            authors: normalize_string_list(authors),
            tags: normalize_string_list(tags),
            paths: normalize_string_list(paths),
        })
    }
}
