use uuid::Uuid;

use crate::{
    dto::FilterName,
    error::{InputError, RepositoryError},
};

#[derive(Debug, Clone)]
pub struct UpdateRepositoryCommitFilterRequest {
    pub filter_id: Uuid,
    pub name: FilterName,
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
}

impl UpdateRepositoryCommitFilterRequest {
    pub fn new(
        filter_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            filter_id,
            name: FilterName::try_new(name).map_err(|e| InputError::new("filter name", e))?,
            authors: normalize(authors),
            tags: normalize(tags),
            paths: normalize(paths),
        })
    }
}

fn normalize(values: Option<Vec<String>>) -> Option<Vec<String>> {
    values.map(|v| {
        v.into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    })
}
