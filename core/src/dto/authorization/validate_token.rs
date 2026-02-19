use uuid::Uuid;

use crate::model::token::TokenType;

#[derive(Debug, Clone)]
pub struct ValidateTokenRequest {
    pub token: String,
    pub token_type: TokenType,
}

#[derive(Debug, Clone)]
pub struct ValidateTokenResponse {
    pub principal_id: Uuid,
}
