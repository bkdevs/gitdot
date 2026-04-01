use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::{endpoint::auth::github::exchange as api, resource::auth::AuthTokensResource};
use gitdot_core::dto::ExchangeGitHubCodeRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{ClientIp, UserAgent},
};

pub async fn exchange_github_code(
    State(state): State<AppState>,
    UserAgent(user_agent): UserAgent,
    ClientIp(ip_address): ClientIp,
    Json(body): Json<api::ExchangeGitHubCodeRequest>,
) -> Result<AppResponse<AuthTokensResource>, AppError> {
    let request =
        ExchangeGitHubCodeRequest::new(body.code, body.state, user_agent, ip_address.as_deref());
    state
        .authentication_service
        .exchange_github_code(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
