use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_core::dto::PollTokenRequest;

use crate::app::{AppError, AppResponse, AppState};
use crate::dto::{PollTokenServerRequest, TokenServerResponse};

#[axum::debug_handler]
pub async fn poll_token(
    State(state): State<AppState>,
    Json(body): Json<PollTokenServerRequest>,
) -> Result<AppResponse<TokenServerResponse>, AppError> {
    let request = PollTokenRequest {
        device_code: body.device_code,
        client_id: body.client_id,
    };
    state
        .oauth_service
        .poll_token(request)
        .await
        .map_err(AppError::from)
        .map(|token| AppResponse::new(StatusCode::CREATED, token.into()))
}
