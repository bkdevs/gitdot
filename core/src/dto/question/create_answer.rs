use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateAnswerRequest {
    pub question_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
}

impl CreateAnswerRequest {
    pub fn new(question_id: Uuid, author_id: Uuid, body: String) -> Self {
        Self {
            question_id,
            author_id,
            body,
        }
    }
}
