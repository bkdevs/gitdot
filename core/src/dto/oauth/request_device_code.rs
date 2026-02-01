#[derive(Debug, Clone)]
pub struct DeviceCodeRequest {
    pub client_id: String,
}

impl DeviceCodeRequest {
    pub fn new(client_id: String) -> Self {
        Self { client_id }
    }
}
