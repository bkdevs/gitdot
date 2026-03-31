use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::auth::email::send as api;
use gitdot_core::dto::SendAuthEmailRequest;

use crate::app::{AppState, error::AppError};

pub async fn send_auth_email(
    State(state): State<AppState>,
    Json(body): Json<api::SendAuthEmailRequest>,
) -> Result<StatusCode, AppError> {
    let request = SendAuthEmailRequest::new(&body.email)?;
    state
        .authentication_service
        .send_auth_email(request)
        .await?;
    Ok(StatusCode::OK)
}
