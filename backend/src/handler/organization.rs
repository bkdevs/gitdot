mod add_member;
mod create_organization;
mod get_organization_repositories;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use add_member::add_member;
use create_organization::create_organization;
use get_organization_repositories::get_organization_repositories;

pub fn create_organization_router() -> Router<AppState> {
    Router::new()
        .route("/organization/{org_name}", post(create_organization))
        .route("/organization/{org_name}/member", post(add_member))
        .route(
            "/organization/{org_name}/repositories",
            get(get_organization_repositories),
        )
}
