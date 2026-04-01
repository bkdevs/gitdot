use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::auth::logout as api;
use gitdot_core::dto::LogoutRequest;

use crate::app::{AppResponse, AppState, error::AppError};

pub async fn logout(
    State(state): State<AppState>,
    Json(body): Json<api::LogoutRequest>,
) -> Result<AppResponse<()>, AppError> {
    let request = LogoutRequest {
        refresh_token: body.refresh_token,
    };
    state
        .authentication_service
        .logout(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::NO_CONTENT, ()))
}
