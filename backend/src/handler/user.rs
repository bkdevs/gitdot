mod get_current_user;
mod get_user;
mod list_user_repositories;
mod validate_name;

use axum::{
    Router,
    routing::{get, head},
};

use crate::app::AppState;

use get_current_user::get_current_user;
use get_user::get_user;
use list_user_repositories::list_user_repositories;
use validate_name::validate_name;

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/user", get(get_current_user))
        .route("/user/{user_name}", get(get_user))
        .route("/user/{user_name}", head(validate_name))
        .route(
            "/user/{user_name}/repositories",
            get(list_user_repositories),
        )
}
