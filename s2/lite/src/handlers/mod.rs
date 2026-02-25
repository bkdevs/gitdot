pub mod v1;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::backend::Backend;

pub fn router() -> axum::Router<Backend> {
    axum::Router::new()
        .route(/* bw compat */ "/ping", axum::routing::get(health))
        .route("/health", axum::routing::get(health))
        .nest("/v1", v1::router())
}

async fn health(State(backend): State<Backend>) -> Response {
    match backend.db_status().await {
        Ok(()) => "OK".into_response(),
        Err(err) => (StatusCode::SERVICE_UNAVAILABLE, format!("{err:?}")).into_response(),
    }
}
