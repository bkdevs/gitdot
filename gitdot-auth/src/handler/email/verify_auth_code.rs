use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::{
    endpoint::auth::email::verify_auth_code as api, resource::auth::AuthTokensResource,
};
use gitdot_axum::extract::{ClientIp, UserAgent};
use gitdot_core::dto::VerifyAuthCodeRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

pub async fn verify_auth_code(
    State(state): State<AppState>,
    UserAgent(user_agent): UserAgent,
    ClientIp(ip_address): ClientIp,
    Json(body): Json<api::VerifyAuthCodeRequest>,
) -> Result<AppResponse<AuthTokensResource>, AppError> {
    let request = VerifyAuthCodeRequest::new(body.code, user_agent, ip_address.as_deref());
    state
        .session_service
        .verify_auth_code(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
