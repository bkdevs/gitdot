mod create_organization;

use axum::{Router, routing::post};

use crate::app::AppState;

use create_organization::create_organization;

pub fn create_organization_router() -> Router<AppState> {
    Router::new().route("/organization/{org_name}", post(create_organization))
}
