use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateAnswerCommentRequest {
    pub author_id: Uuid,
    pub answer_id: Uuid,
    pub body: String,
}

impl CreateAnswerCommentRequest {
    pub fn new(author_id: Uuid, answer_id: Uuid, body: String) -> Self {
        Self {
            author_id,
            answer_id,
            body,
        }
    }
}
