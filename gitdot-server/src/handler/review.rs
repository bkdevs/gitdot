mod add_reviewer;
mod get_review;
mod get_reviews;
mod remove_reviewer;

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::app::AppState;

use add_reviewer::add_reviewer;
use get_review::get_review;
use get_reviews::get_reviews;
use remove_reviewer::remove_reviewer;

pub fn create_review_router() -> Router<AppState> {
    Router::new()
        .route(
            "/repository/{owner}/{repo}/review/{number}",
            get(get_review),
        )
        .route("/repository/{owner}/{repo}/reviews", get(get_reviews))
        .route(
            "/repository/{owner}/{repo}/review/{number}/reviewer",
            post(add_reviewer),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/reviewer/{reviewer_name}",
            delete(remove_reviewer),
        )
}
