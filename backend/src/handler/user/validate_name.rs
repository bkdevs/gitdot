use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::ValidateNameRequest;

use crate::app::{AppError, AppResponse, AppState};

pub async fn validate_name(
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<()>, AppError> {
    let validate_request = ValidateNameRequest::new(&user_name)?;
    state
        .user_service
        .validate_name(validate_request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
