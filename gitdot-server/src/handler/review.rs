mod add_reviewer;
mod get_review;
mod get_review_diff;
mod get_reviews;
mod publish_review;
mod remove_reviewer;

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::app::AppState;

use add_reviewer::add_reviewer;
use get_review::get_review;
use get_review_diff::get_review_diff;
use get_reviews::get_reviews;
use publish_review::publish_review;
use remove_reviewer::remove_reviewer;

pub fn create_review_router() -> Router<AppState> {
    Router::new()
        .route(
            "/repository/{owner}/{repo}/review/{number}",
            get(get_review),
        )
        .route("/repository/{owner}/{repo}/reviews", get(get_reviews))
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}",
            get(get_review_diff),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/publish",
            post(publish_review),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/reviewer",
            post(add_reviewer),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/reviewer/{reviewer_name}",
            delete(remove_reviewer),
        )
}
