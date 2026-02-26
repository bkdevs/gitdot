use uuid::Uuid;

use super::MigrationResponse;

#[derive(Debug, Clone)]
pub struct ListMigrationsRequest {
    pub user_id: Uuid,
}

impl ListMigrationsRequest {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

pub type ListMigrationsResponse = Vec<MigrationResponse>;
