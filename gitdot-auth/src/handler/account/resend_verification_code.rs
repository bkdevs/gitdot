use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::auth::account::resend_verification_code as api;
use gitdot_axum::extract::Principal;
use gitdot_core::dto::ResendVerificationCodeRequest;

use crate::app::{AppError, AppResponse, AppState};

pub async fn resend_verification_code(
    principal: Principal,
    State(state): State<AppState>,
    Json(body): Json<api::ResendVerificationCodeRequest>,
) -> Result<AppResponse<()>, AppError> {
    let request = ResendVerificationCodeRequest::new(principal.id, &body.email)?;
    state
        .account_service
        .resend_code(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
