use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::auth::email::verify as api;
use gitdot_core::dto::VerifyAuthCodeRequest;

use crate::{
    app::{AppResponse, AppState, error::AppError},
    dto::IntoApi,
    extract::{ClientIp, UserAgent},
};

pub async fn verify_auth_code(
    State(state): State<AppState>,
    UserAgent(user_agent): UserAgent,
    ClientIp(ip_address): ClientIp,
    Json(body): Json<api::VerifyAuthCodeRequest>,
) -> Result<AppResponse<api::VerifyAuthCodeResponse>, AppError> {
    let request = VerifyAuthCodeRequest::new(body.code, user_agent, ip_address.as_deref());
    state
        .authentication_service
        .verify_auth_code(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
