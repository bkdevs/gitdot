use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpdateQuestionRequest {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

impl UpdateQuestionRequest {
    pub fn new(id: Uuid, title: String, body: String) -> Self {
        Self { id, title, body }
    }
}
