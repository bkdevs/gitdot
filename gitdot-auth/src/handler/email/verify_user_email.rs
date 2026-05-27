use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::auth::email::verify_user_email as api;
use gitdot_axum::extract::Principal;
use gitdot_core::dto::VerifyUserEmailRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

pub async fn verify_user_email(
    principal: Principal,
    State(state): State<AppState>,
    Json(body): Json<api::VerifyUserEmailRequest>,
) -> Result<AppResponse<api::VerifyUserEmailResponse>, AppError> {
    let request = VerifyUserEmailRequest::new(principal.id, &body.email, &body.code)?;
    state
        .email_verification_service
        .verify_email(request)
        .await
        .map_err(AppError::from)
        .map(|email| AppResponse::new(StatusCode::OK, email.into_api()))
}
