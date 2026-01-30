use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use gitdot_core::dto::PollTokenRequest;

use crate::app::{AppError, AppResponse, AppState};

#[derive(Deserialize)]
pub struct PollTokenServerRequest {
    pub device_code: String,
}

#[derive(Serialize, PartialEq)]
pub struct TokenServerResponse {
    pub access_token: String,
    pub token_type: String,
}

#[axum::debug_handler]
pub async fn poll_token(
    State(state): State<AppState>,
    Json(body): Json<PollTokenServerRequest>,
) -> Result<AppResponse<TokenServerResponse>, AppError> {
    let request = PollTokenRequest {
        device_code: body.device_code,
    };

    state
        .oauth_service
        .poll_token(request)
        .await
        .map_err(AppError::from)
        .map(|response| {
            AppResponse::new(
                StatusCode::OK,
                TokenServerResponse {
                    access_token: response.access_token,
                    token_type: response.token_type,
                },
            )
        })
}
