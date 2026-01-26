use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetQuestionsRequest {
    pub repository_id: Uuid,
}

impl GetQuestionsRequest {
    pub fn new(repository_id: Uuid) -> Self {
        Self { repository_id }
    }
}
