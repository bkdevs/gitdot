mod add_member;
mod create_organization;
mod list_organization_repositories;

use crate::app::AppState;
use axum::{
    Router,
    routing::{get, post},
};

use add_member::add_member;
use create_organization::create_organization;
use list_organization_repositories::list_organization_repositories;

pub fn create_organization_router() -> Router<AppState> {
    Router::new()
        .route("/organization/{org_name}", post(create_organization))
        .route("/organization/{org_name}/member", post(add_member))
        .route(
            "/organization/{org_name}/repositories",
            get(list_organization_repositories),
        )
}
