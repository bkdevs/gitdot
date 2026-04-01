mod email;
mod github;
mod logout;
mod refresh_session;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use email::{send::send_auth_email, verify::verify_auth_code};
use github::redirect::redirect_to_github_auth;
use logout::logout;
use refresh_session::refresh_session;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/email/send", post(send_auth_email))
        .route("/auth/email/verify", post(verify_auth_code))
        .route("/auth/github/redirect", get(redirect_to_github_auth))
        .route("/auth/refresh", post(refresh_session))
        .route("/auth/logout", post(logout))
}
