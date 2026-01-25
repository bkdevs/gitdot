use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetQuestionsForRepositoryRequest {
    pub repository_id: Uuid,
}

impl GetQuestionsForRepositoryRequest {
    pub fn new(repository_id: Uuid) -> Self {
        Self { repository_id }
    }
}
