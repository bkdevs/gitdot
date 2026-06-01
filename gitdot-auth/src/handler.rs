mod account;
mod device;
mod email;
mod github;
mod logout;
mod refresh_session;
mod slack;

use axum::{
    Router, middleware,
    routing::{delete, get, post},
};
use gitdot_axum::middleware::verify_vercel_oidc;

use crate::app::AppState;

use account::{add_user_email, delete_account, verify_user_email};
use device::{authorize_device, create_device_code, poll_token};
use email::{send_auth_email, verify_auth_code};
use github::{exchange_github_code, redirect_to_github_auth};
use logout::logout;
use refresh_session::refresh_session;
use slack::link_slack_account;

pub fn create_auth_router(state: AppState) -> Router<AppState> {
    let cli_routes = Router::new()
        .route("/auth/device/code", post(create_device_code))
        .route("/auth/device/token", post(poll_token));

    let web_routes = Router::new()
        .route("/auth/device/authorize", post(authorize_device))
        .route("/auth/account", delete(delete_account))
        .route("/auth/account/add-email", post(add_user_email))
        .route("/auth/account/verify-email", post(verify_user_email))
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
