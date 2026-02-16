use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessPostReceiveServerRequest {
    pub old_sha: String,
    pub new_sha: String,
    pub ref_name: String,
}
