use uuid::Uuid;

use crate::{
    error::{InputError, ReviewError},
    model::CommentSide,
};

use crate::dto::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct JudgeReviewDiffRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub position: i32,
    pub reviewer_id: Uuid,
    pub action: JudgeAction,
    pub comments: Vec<DiffComment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudgeAction {
    Approve,
    RequestChanges,
    Comment,
}

#[derive(Debug, Clone)]
pub struct DiffComment {
    pub body: String,
    pub parent_id: Option<Uuid>,
    pub file_path: Option<String>,
    pub line_number_start: Option<i32>,
    pub line_number_end: Option<i32>,
    pub side: Option<CommentSide>,
}

impl JudgeReviewDiffRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        position: i32,
        reviewer_id: Uuid,
        action: &str,
        comments: Vec<DiffComment>,
    ) -> Result<Self, ReviewError> {
        let action = match action {
            "approve" => JudgeAction::Approve,
            "request_changes" => JudgeAction::RequestChanges,
            "comment" => JudgeAction::Comment,
            _ => {
                return Err(InputError::new(
                    "comment",
                    format!(
                        "Invalid action: {action}. Must be approve, request_changes, or comment"
                    ),
                )
                .into());
            }
        };

        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            position,
            reviewer_id,
            action,
            comments,
        })
    }
}

impl DiffComment {
    pub fn new(
        body: String,
        parent_id: Option<Uuid>,
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
                        "comment",
                        format!("Invalid side: {s}. Must be old or new"),
                    )
                    .into()),
                }
            })
            .transpose()?;

        Ok(Self {
            body,
            parent_id,
            file_path,
            line_number_start,
            line_number_end,
            side,
        })
    }
}
