use axum::Router;

use crate::app::AppState;

pub fn create_auth_router() -> Router<AppState> {
    Router::new()
}
