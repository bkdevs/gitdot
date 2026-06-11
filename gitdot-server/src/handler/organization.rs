mod add_member;
mod create_organization;
mod follow_organization;
mod get_organization;
mod list_organization_repositories;
mod list_organizations;
mod unfollow_organization;
mod update_member;
mod update_organization;
mod upload_organization_image;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, patch, post},
};

use crate::app::AppState;

use add_member::add_member;
use create_organization::create_organization;
use follow_organization::follow_organization;
use get_organization::get_organization;
use list_organization_repositories::list_organization_repositories;
use list_organizations::list_organizations;
use unfollow_organization::unfollow_organization;
use update_member::update_member;
use update_organization::update_organization;
use upload_organization_image::upload_organization_image;

pub fn create_organization_router() -> Router<AppState> {
    Router::new()
        .route("/organizations", get(list_organizations))
        .route(
            "/organization/{org_name}",
            get(get_organization)
                .post(create_organization)
                .patch(update_organization),
        )
        .route(
            "/organization/{org_name}/image",
            post(upload_organization_image).layer(DefaultBodyLimit::max(5 * 1024 * 1024)),
        )
        .route(
            "/organization/{org_name}/follow",
            post(follow_organization).delete(unfollow_organization),
        )
        .route("/organization/{org_name}/member", post(add_member))
        .route(
            "/organization/{org_name}/member/{member_id}",
            patch(update_member),
        )
        .route(
            "/organization/{org_name}/repositories",
            get(list_organization_repositories),
        )
}
