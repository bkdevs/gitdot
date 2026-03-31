use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::auth::email::send as api;
use gitdot_core::dto::SendAuthEmailRequest;

use crate::app::{AppResponse, AppState, error::AppError};

pub async fn send_auth_email(
    State(state): State<AppState>,
    Json(body): Json<api::SendAuthEmailRequest>,
) -> Result<AppResponse<()>, AppError> {
    let request = SendAuthEmailRequest::new(&body.email)?;
    state
        .authentication_service
        .send_auth_email(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
