mod get_user;
mod list_user_repositories;

use axum::{Router, routing::get};

use crate::app::AppState;

use get_user::get_user;
use list_user_repositories::list_user_repositories;

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/user/{user_name}", get(get_user))
        .route(
            "/user/{user_name}/repositories",
            get(list_user_repositories),
        )
}
