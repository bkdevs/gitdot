use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AnswerAuthorizationRequest {
    pub user_id: Uuid,
    pub answer_id: Uuid,
}

impl AnswerAuthorizationRequest {
    pub fn new(user_id: Uuid, answer_id: Uuid) -> Self {
        Self { user_id, answer_id }
    }
}
