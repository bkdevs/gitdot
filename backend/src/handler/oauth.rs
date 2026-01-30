mod authorize_device;
mod poll_token;
mod request_device_code;

use axum::{Router, routing::post};

use crate::app::AppState;

use authorize_device::authorize_device;
use poll_token::poll_token;
use request_device_code::request_device_code;

pub fn create_oauth_router() -> Router<AppState> {
    Router::new()
        .route("/oauth/device", post(request_device_code))
        .route("/oauth/token", post(poll_token))
        .route("/oauth/authorize", post(authorize_device))
}
