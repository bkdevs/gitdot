use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UpdateAnswerRequest {
    pub id: Uuid,
    pub body: String,
}

impl UpdateAnswerRequest {
    pub fn new(id: Uuid, body: String) -> Self {
        Self { id, body }
    }
}
