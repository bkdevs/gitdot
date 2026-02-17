use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct ValidateTokenResponse {
    pub user_id: Uuid,
}
