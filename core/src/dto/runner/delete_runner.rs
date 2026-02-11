use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DeleteRunnerRequest {
    pub id: Uuid,
}

impl DeleteRunnerRequest {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
