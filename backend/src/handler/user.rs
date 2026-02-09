mod get_current_user;
mod get_user;
mod has_user;
mod list_user_repositories;
mod update_current_user;

use axum::{Router, routing::get};

use crate::app::AppState;

use get_current_user::get_current_user;
use get_user::get_user;
use has_user::has_user;
use list_user_repositories::list_user_repositories;
use update_current_user::update_current_user;

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/user", get(get_current_user).patch(update_current_user))
        .route("/user/{user_name}", get(get_user).head(has_user))
        .route(
            "/user/{user_name}/repositories",
            get(list_user_repositories),
        )
}
