use uuid::Uuid;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
    model::CommentSide,
};


#[derive(Debug, Clone)]
pub struct ReviewCommentInput {
    pub diff_id: Uuid,
    pub revision_id: Uuid,
    pub body: String,
    pub file_path: Option<String>,
    pub line_number_start: Option<i32>,
    pub line_number_end: Option<i32>,
    pub start_character: Option<i32>,
    pub end_character: Option<i32>,
    pub side: Option<CommentSide>,
}

#[derive(Debug, Clone)]
pub struct PublishReviewCommentsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub author_id: Uuid,
    pub comments: Vec<ReviewCommentInput>,
}

impl PublishReviewCommentsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        author_id: Uuid,
        comments: Vec<(
            Uuid,
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
                    diff_id,
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
                        diff_id,
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
            author_id,
            comments,
        })
    }
}
