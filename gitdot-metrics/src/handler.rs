mod metrics;

use axum::{Router, middleware, routing::post};

use gitdot_axum::middleware::verify_vercel_oidc;

use crate::app::AppState;

use metrics::web_vital::log_web_vital;

pub fn create_metrics_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/metrics/web-vital", post(log_web_vital))
        .route_layer(middleware::from_fn_with_state(
            state.vercel_oidc_config,
            verify_vercel_oidc,
        ))
}
