use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::{InputError, ReviewError};

use crate::dto::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct ListReviewsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub viewer_id: Option<Uuid>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl ListReviewsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        viewer_id: Option<Uuid>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            viewer_id,
            from,
            to,
        })
    }
}
