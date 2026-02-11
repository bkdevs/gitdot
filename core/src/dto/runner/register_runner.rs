use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RegisterRunnerRequest {
    pub id: Uuid,
}

impl RegisterRunnerRequest {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
