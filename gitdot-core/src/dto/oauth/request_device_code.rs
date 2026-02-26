#[derive(Debug, Clone)]
pub struct DeviceCodeRequest {
    pub client_id: String,
    pub verification_uri: String,
}

impl DeviceCodeRequest {
    pub fn new(client_id: String, verification_uri: String) -> Self {
        Self {
            client_id,
            verification_uri,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_code_request_new() {
        let request = DeviceCodeRequest::new(
            "my-client".to_string(),
            "https://example.com/verify".to_string(),
        );

        assert_eq!(request.client_id, "my-client");
        assert_eq!(request.verification_uri, "https://example.com/verify");
    }

    #[test]
    fn device_code_response_fields() {
        let response = DeviceCodeResponse {
            device_code: "device123".to_string(),
            user_code: "ABCD-1234".to_string(),
            verification_uri: "https://example.com/verify".to_string(),
            expires_in: 900,
            interval: 5,
        };

        assert_eq!(response.device_code, "device123");
        assert_eq!(response.user_code, "ABCD-1234");
        assert_eq!(response.verification_uri, "https://example.com/verify");
        assert_eq!(response.expires_in, 900);
        assert_eq!(response.interval, 5);
    }
}
