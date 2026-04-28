use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
    model::CommentSide,
};

use super::review_review_diff::ReviewCommentInput;

#[derive(Debug, Clone)]
pub struct CreateReviewCommentsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub position: i32,
    pub reviewer_id: Uuid,
    pub comments: Vec<ReviewCommentInput>,
}

impl CreateReviewCommentsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        position: i32,
        reviewer_id: Uuid,
        comments: Vec<(
            Uuid,
            String,
            Option<String>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<String>,
        )>,
    ) -> Result<Self, ReviewError> {
        let comments = comments
            .into_iter()
            .map(
                |(
                    revision_id,
                    body,
                    file_path,
                    line_number_start,
                    line_number_end,
                    start_character,
                    end_character,
                    side,
                )| {
                    let side = side
                        .as_deref()
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

                    Ok(ReviewCommentInput {
                        revision_id,
                        body,
                        file_path,
                        line_number_start,
                        line_number_end,
                        start_character,
                        end_character,
                        side,
                    })
                },
            )
            .collect::<Result<Vec<_>, ReviewError>>()?;

        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            position,
            reviewer_id,
            comments,
        })
    }
}
