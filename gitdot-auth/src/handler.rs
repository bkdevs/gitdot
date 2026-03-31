mod send_auth_email;

use axum::{Router, routing::post};

use crate::app::AppState;

use send_auth_email::send_auth_email;

pub fn create_auth_router() -> Router<AppState> {
    Router::new().route("/auth/email/send", post(send_auth_email))
}
