use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateQuestionRequest {
    pub author_id: Uuid,
    pub repository_id: Uuid,
    pub title: String,
    pub body: String,
}

impl CreateQuestionRequest {
    pub fn new(author_id: Uuid, repository_id: Uuid, title: String, body: String) -> Self {
        Self {
            author_id,
            repository_id,
            title,
            body,
        }
    }
}
