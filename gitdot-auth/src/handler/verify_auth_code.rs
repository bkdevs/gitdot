use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, SameSite};

use gitdot_api::endpoint::auth::email::verify as api;
use gitdot_core::dto::{VerifyAuthCodeRequest, VerifyAuthCodeResponse};

use crate::{
    app::{AppResponse, AppState, error::AppError},
    dto::IntoApi,
    extract::{ClientIp, UserAgent},
};

const ACCESS_TOKEN_MAX_AGE: i64 = 3600;
const REFRESH_TOKEN_MAX_AGE: i64 = 30 * 24 * 3600;

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
        .map(into_response)
}

fn into_response(response: VerifyAuthCodeResponse) -> AppResponse<api::VerifyAuthCodeResponse> {
    let access_cookie = Cookie::build(("gd_access_token", response.access_token.clone()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(time::Duration::seconds(ACCESS_TOKEN_MAX_AGE));

    let refresh_cookie = Cookie::build(("gd_refresh_token", response.refresh_token.clone()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/auth/refresh")
        .max_age(time::Duration::seconds(REFRESH_TOKEN_MAX_AGE));

    AppResponse::new(StatusCode::OK, response.into_api())
        .with_header("set-cookie", &access_cookie.to_string())
        .with_header("set-cookie", &refresh_cookie.to_string())
}
