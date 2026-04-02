mod device;
mod email;
mod github;
mod logout;
mod refresh_session;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use device::{
    authorize_device::authorize_device, create_device_code::create_device_code,
    poll_token::poll_token,
};
use email::{send::send_auth_email, verify::verify_auth_code};
use github::{exchange::exchange_github_code, redirect::redirect_to_github_auth};
use logout::logout;
use refresh_session::refresh_session;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/email/send", post(send_auth_email))
        .route("/auth/email/verify", post(verify_auth_code))
        .route("/auth/github/redirect", get(redirect_to_github_auth))
        .route("/auth/github/exchange", post(exchange_github_code))
        .route("/auth/device/code", post(create_device_code))
        .route("/auth/device/token", post(poll_token))
        .route("/auth/device/authorize", post(authorize_device))
        .route("/auth/refresh", post(refresh_session))
        .route("/auth/logout", post(logout))
}
