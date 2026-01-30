use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeDeviceRequest {
    pub user_code: String,
    pub user_id: Uuid,
}
