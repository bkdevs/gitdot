mod refresh_session;
mod send_auth_email;
mod verify_auth_code;

use axum::{Router, routing::post};

use crate::app::AppState;

use refresh_session::refresh_session;
use send_auth_email::send_auth_email;
use verify_auth_code::verify_auth_code;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/email/send", post(send_auth_email))
        .route("/auth/email/verify", post(verify_auth_code))
        .route("/auth/refresh", post(refresh_session))
}
