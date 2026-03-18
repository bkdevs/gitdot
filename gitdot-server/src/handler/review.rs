mod add_reviewer;
mod get_review;
mod get_review_diff;
mod list_reviews;
mod publish_review;
mod remove_reviewer;
mod resolve_review_comment;
mod submit_review;
mod update_review_comment;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::app::AppState;

use add_reviewer::add_reviewer;
use get_review::get_review;
use get_review_diff::get_review_diff;
use list_reviews::list_reviews;
use publish_review::publish_review;
use remove_reviewer::remove_reviewer;
use resolve_review_comment::resolve_review_comment;
use submit_review::submit_review;
use update_review_comment::update_review_comment;

pub fn create_review_router() -> Router<AppState> {
    Router::new()
        .route(
            "/repository/{owner}/{repo}/review/{number}",
            get(get_review),
        )
        .route("/repository/{owner}/{repo}/reviews", get(list_reviews))
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
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}/submit",
            post(submit_review),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/comment/{comment_id}",
            patch(update_review_comment),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/comment/{comment_id}/resolve",
            post(resolve_review_comment),
        )
}
