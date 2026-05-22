mod device;
mod email;
mod github;
mod logout;
mod refresh_session;
mod slack;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use gitdot_axum::middleware::verify_vercel_oidc;

use crate::app::AppState;

use device::{
    authorize_device::authorize_device, create_device_code::create_device_code,
    poll_token::poll_token,
};
use email::{send::send_auth_email, verify::verify_auth_code};
use github::{exchange::exchange_github_code, redirect::redirect_to_github_auth};
use logout::logout;
use refresh_session::refresh_session;
use slack::link::link_slack_account;

pub fn create_auth_router(state: AppState) -> Router<AppState> {
    let cli_routes = Router::new()
        .route("/auth/device/code", post(create_device_code))
        .route("/auth/device/token", post(poll_token));

    let web_routes = Router::new()
        .route("/auth/device/authorize", post(authorize_device))
        .route("/auth/email/send", post(send_auth_email))
        .route("/auth/email/verify", post(verify_auth_code))
        .route("/auth/github/redirect", get(redirect_to_github_auth))
        .route("/auth/github/exchange", post(exchange_github_code))
        .route("/auth/slack/link", post(link_slack_account))
        .route("/auth/refresh", post(refresh_session))
        .route("/auth/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(
            state.vercel_oidc_config,
            verify_vercel_oidc,
        ));

    Router::new().merge(cli_routes).merge(web_routes)
}
