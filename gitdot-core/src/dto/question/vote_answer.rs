use uuid::Uuid;

use crate::error::QuestionError;

#[derive(Debug, Clone)]
pub struct VoteAnswerRequest {
    pub answer_id: Uuid,
    pub user_id: Uuid,
    pub value: i16,
}

impl VoteAnswerRequest {
    pub fn new(answer_id: Uuid, user_id: Uuid, value: i16) -> Result<Self, QuestionError> {
        if !(-1..=1).contains(&value) {
            return Err(QuestionError::InvalidVoteValue(value));
        }
        Ok(Self {
            answer_id,
            user_id,
            value,
        })
    }
}
