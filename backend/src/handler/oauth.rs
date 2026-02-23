mod authorize_device;
mod create_device_code;
mod poll_token;

use axum::{Router, routing::post};

use crate::app::AppState;

use authorize_device::authorize_device;
use create_device_code::create_device_code;
use poll_token::poll_token;

pub fn create_oauth_router() -> Router<AppState> {
    Router::new()
        .route("/oauth/device", post(create_device_code))
        .route("/oauth/token", post(poll_token))
        .route("/oauth/authorize", post(authorize_device))
}
