use axum::{
    Json,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::{Cookie, SameSite};

use gitdot_api::{ApiResource, resource::auth::AuthTokensResource};
use gitdot_core::dto::AuthTokensResponse;

use crate::{
    consts::{ACCESS_TOKEN_COOKIE_NAME, REFRESH_TOKEN_COOKIE_NAME},
    dto::IntoApi,
};

#[derive(Debug, Clone)]
pub struct AppResponse<T: ApiResource> {
    status: StatusCode,
    headers: HeaderMap,
    data: T,
}

impl<T: ApiResource> AppResponse<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            data,
        }
    }

    fn with_header(mut self, name: &str, value: &str) -> Self {
        if let (Ok(name), Ok(value)) = (name.parse::<HeaderName>(), value.parse::<HeaderValue>()) {
            self.headers.append(name, value);
        }
        self
    }
}

impl AppResponse<AuthTokensResource> {
    pub fn auth(response: AuthTokensResponse) -> Self {
        let access_cookie =
            Cookie::build((ACCESS_TOKEN_COOKIE_NAME, response.access_token.clone()))
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .path("/")
                .max_age(time::Duration::seconds(
                    response.access_token_expires_in as i64,
                ));

        let refresh_cookie =
            Cookie::build((REFRESH_TOKEN_COOKIE_NAME, response.refresh_token.clone()))
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .path("/auth/refresh")
                .max_age(time::Duration::seconds(
                    response.refresh_token_expires_in as i64,
                ));

        Self::new(StatusCode::OK, response.into_api())
            .with_header("set-cookie", &access_cookie.to_string())
            .with_header("set-cookie", &refresh_cookie.to_string())
    }
}

impl AppResponse<()> {
    pub fn clear_auth_cookies() -> Self {
        let access_cookie = Cookie::build((ACCESS_TOKEN_COOKIE_NAME, ""))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .path("/")
            .max_age(time::Duration::ZERO);

        let refresh_cookie = Cookie::build((REFRESH_TOKEN_COOKIE_NAME, ""))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .path("/auth/refresh")
            .max_age(time::Duration::ZERO);

        Self::new(StatusCode::NO_CONTENT, ())
            .with_header("set-cookie", &access_cookie.to_string())
            .with_header("set-cookie", &refresh_cookie.to_string())
    }
}

impl<T: ApiResource> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        (self.status, self.headers, Json(self.data)).into_response()
    }
}
