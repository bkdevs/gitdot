#[derive(Debug, Clone)]
pub struct PollTokenRequest {
    pub device_code: String,
    pub client_id: String,
}

impl PollTokenRequest {
    pub fn new(device_code: String, client_id: String) -> Self {
        Self {
            device_code,
            client_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub user_email: String,
    pub user_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_token_request_new() {
        let request = PollTokenRequest::new("device123".to_string(), "my-client".to_string());

        assert_eq!(request.device_code, "device123");
        assert_eq!(request.client_id, "my-client");
    }

    #[test]
    fn token_response_fields() {
        let response = TokenResponse {
            access_token: "token123".to_string(),
            user_email: "user@example.com".to_string(),
            user_name: "johndoe".to_string(),
        };

        assert_eq!(response.access_token, "token123");
        assert_eq!(response.user_email, "user@example.com");
        assert_eq!(response.user_name, "johndoe");
    }
}
