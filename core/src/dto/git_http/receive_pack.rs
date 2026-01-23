#[derive(Debug, Clone)]
pub struct ReceivePackRequest {
    pub owner: String,
    pub repo: String,
    pub content_type: String,
    pub body: Vec<u8>,
}
