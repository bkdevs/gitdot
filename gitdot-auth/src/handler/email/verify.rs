use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::{endpoint::auth::email::verify as api, resource::auth::AuthTokensResource};
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
) -> Result<AppResponse<AuthTokensResource>, AppError> {
    let request = VerifyAuthCodeRequest::new(body.code, user_agent, ip_address.as_deref());
    state
        .authentication_service
        .verify_auth_code(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
