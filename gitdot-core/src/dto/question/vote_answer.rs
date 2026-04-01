use uuid::Uuid;

use crate::error::{InputError, QuestionError};

#[derive(Debug, Clone)]
pub struct VoteAnswerRequest {
    pub answer_id: Uuid,
    pub user_id: Uuid,
    pub value: i16,
}

impl VoteAnswerRequest {
    pub fn new(answer_id: Uuid, user_id: Uuid, value: i16) -> Result<Self, QuestionError> {
        if !(-1..=1).contains(&value) {
            return Err(
                InputError::new("vote value", format!("{value}. Must be -1, 0, or 1")).into(),
            );
        }
        Ok(Self {
            answer_id,
            user_id,
            value,
        })
    }
}
