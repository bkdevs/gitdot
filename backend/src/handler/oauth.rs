mod authorize_device;
mod get_device_code;
mod poll_token;

use axum::{Router, routing::post};

use crate::app::AppState;

use authorize_device::authorize_device;
use get_device_code::get_device_code;
use poll_token::poll_token;

pub fn create_oauth_router() -> Router<AppState> {
    Router::new()
        .route("/oauth/device", post(get_device_code))
        .route("/oauth/token", post(poll_token))
        .route("/oauth/authorize", post(authorize_device))
}
