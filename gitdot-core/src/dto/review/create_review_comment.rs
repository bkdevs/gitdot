use uuid::Uuid;

use crate::{error::ReviewError, model::CommentSide};

use super::super::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct CreateReviewCommentRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub author_id: Uuid,
    pub body: String,
    pub diff_id: Option<Uuid>,
    pub revision_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub file_path: Option<String>,
    pub line_number: Option<i32>,
    pub side: Option<CommentSide>,
}

impl CreateReviewCommentRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        author_id: Uuid,
        body: &str,
        diff_id: Option<Uuid>,
        revision_id: Option<Uuid>,
        parent_id: Option<Uuid>,
        file_path: Option<&str>,
        line_number: Option<i32>,
        side: Option<&str>,
    ) -> Result<Self, ReviewError> {
        let side = side
            .map(|s| match s {
                "old" => Ok(CommentSide::Old),
                "new" => Ok(CommentSide::New),
                _ => Err(ReviewError::InvalidComment(format!(
                    "invalid side: {s}, must be 'old' or 'new'"
                ))),
            })
            .transpose()?;

        if file_path.is_some() && (line_number.is_none() || side.is_none()) {
            return Err(ReviewError::InvalidComment(
                "file_path requires line_number and side".to_string(),
            ));
        }

        if revision_id.is_some() && diff_id.is_none() {
            return Err(ReviewError::InvalidComment(
                "revision_id requires diff_id".to_string(),
            ));
        }

        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| ReviewError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| ReviewError::InvalidRepositoryName(e.to_string()))?,
            number,
            author_id,
            body: body.to_string(),
            diff_id,
            revision_id,
            parent_id,
            file_path: file_path.map(|s| s.to_string()),
            line_number,
            side,
        })
    }
}
