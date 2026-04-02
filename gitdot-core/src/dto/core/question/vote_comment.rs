use uuid::Uuid;

use crate::error::{InputError, QuestionError};

#[derive(Debug, Clone)]
pub struct VoteCommentRequest {
    pub comment_id: Uuid,
    pub user_id: Uuid,
    pub value: i16,
}

impl VoteCommentRequest {
    pub fn new(comment_id: Uuid, user_id: Uuid, value: i16) -> Result<Self, QuestionError> {
        if !(-1..=1).contains(&value) {
            return Err(
                InputError::new("vote value", format!("{value}. Must be -1, 0, or 1")).into(),
            );
        }
        Ok(Self {
            comment_id,
            user_id,
            value,
        })
    }
}
