use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_api::endpoint::oauth::poll_token as api;
use gitdot_core::dto::PollTokenRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn poll_token(
    State(state): State<AppState>,
    Json(body): Json<api::PollTokenRequest>,
) -> Result<AppResponse<api::PollTokenResponse>, AppError> {
    let request = PollTokenRequest {
        device_code: body.device_code,
        client_id: body.client_id,
    };
    state
        .token_service
        .poll_token(request)
        .await
        .map_err(AppError::from)
        .map(|token| AppResponse::new(StatusCode::CREATED, token.into_api()))
}
