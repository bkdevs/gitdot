use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetQuestionRequest {
    pub id: Uuid,
}

impl GetQuestionRequest {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
