use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateAnswerRequest {
    pub author_id: Uuid,
    pub question_id: Uuid,
    pub body: String,
}

impl CreateAnswerRequest {
    pub fn new(author_id: Uuid, question_id: Uuid, body: String) -> Self {
        Self {
            author_id,
            question_id,
            body,
        }
    }
}
