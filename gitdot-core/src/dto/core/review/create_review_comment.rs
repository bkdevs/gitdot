use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
    model::CommentSide,
};

use super::ReviewId;

#[derive(Debug, Clone)]
pub struct CreateReviewCommentRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub review_id: ReviewId,
    pub author_id: Uuid,
    pub diff_id: Uuid,
    pub revision_id: Uuid,
    pub body: String,
    pub file_path: Option<String>,
    pub line_number_start: Option<i32>,
    pub line_number_end: Option<i32>,
    pub side: Option<CommentSide>,
}

impl CreateReviewCommentRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        owner: &str,
        repo: &str,
        review_id: ReviewId,
        author_id: Uuid,
        diff_id: Uuid,
        revision_id: Uuid,
        body: String,
        file_path: Option<String>,
        line_number_start: Option<i32>,
        line_number_end: Option<i32>,
        side: Option<&str>,
    ) -> Result<Self, ReviewError> {
        let side = side
            .map(|s| -> Result<CommentSide, ReviewError> {
                match s {
                    "old" => Ok(CommentSide::Old),
                    "new" => Ok(CommentSide::New),
                    _ => Err(InputError::new(
                        "side",
                        format!("Invalid side: {s}. Must be old or new"),
                    )
                    .into()),
                }
            })
            .transpose()?;

        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            review_id,
            author_id,
            diff_id,
            revision_id,
            body,
            file_path,
            line_number_start,
            line_number_end,
            side,
        })
    }
}
