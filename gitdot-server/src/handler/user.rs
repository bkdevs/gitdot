mod get_current_user;
mod get_current_user_settings;
mod get_user;
mod has_user;
mod list_user_commits;
mod list_user_organizations;
mod list_user_repositories;
mod list_user_reviews;
mod update_current_user;
mod update_current_user_settings;
mod upload_user_image;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use crate::app::AppState;

use get_current_user::get_current_user;
use get_current_user_settings::get_current_user_settings;
use get_user::get_user;
use has_user::has_user;
use list_user_commits::list_user_commits;
use list_user_organizations::list_user_organizations;
use list_user_repositories::list_user_repositories;
use list_user_reviews::list_user_reviews;
use update_current_user::update_current_user;
use update_current_user_settings::update_current_user_settings;
use upload_user_image::upload_user_image;

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/user", get(get_current_user).patch(update_current_user))
        .route(
            "/user/image",
            post(upload_user_image).layer(DefaultBodyLimit::max(5 * 1024 * 1024)),
        )
        .route(
            "/user/settings",
            get(get_current_user_settings).patch(update_current_user_settings),
        )
        .route("/user/{user_name}", get(get_user).head(has_user))
        .route(
            "/user/{user_name}/repositories",
            get(list_user_repositories),
        )
        .route(
            "/user/{user_name}/organizations",
            get(list_user_organizations),
        )
        .route("/user/{user_name}/reviews", get(list_user_reviews))
        .route("/user/{user_name}/commits", get(list_user_commits))
}
