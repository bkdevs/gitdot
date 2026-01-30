use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollTokenRequest {
    pub device_code: String,
}
