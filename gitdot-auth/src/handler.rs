mod login_with_email;

use axum::{Router, routing::post};

use crate::app::AppState;

use login_with_email::login_with_email;

pub fn create_auth_router() -> Router<AppState> {
    Router::new().route("/auth/login/email", post(login_with_email))
}
