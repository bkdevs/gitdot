use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QuestionAuthorizationRequest {
    pub user_id: Uuid,
    pub question_id: Uuid,
}

impl QuestionAuthorizationRequest {
    pub fn new(user_id: Uuid, question_id: Uuid) -> Self {
        Self {
            user_id,
            question_id,
        }
    }
}
