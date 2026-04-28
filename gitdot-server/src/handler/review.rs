mod add_review_reviewer;
mod approve_review_diff;
mod create_review_comments;
mod reject_review_diff;
mod get_review;
mod get_review_diff;
mod list_reviews;
mod merge_review;
mod merge_review_diff;
mod publish_review;
mod publish_review_diff;
mod remove_review_reviewer;
mod resolve_review_comment;
mod update_review;
mod update_review_comment;
mod update_review_diff;

use crate::app::AppState;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use add_review_reviewer::add_review_reviewer;
use approve_review_diff::approve_review_diff;
use create_review_comments::create_review_comments;
use reject_review_diff::reject_review_diff;
use get_review::get_review;
use get_review_diff::get_review_diff;
use list_reviews::list_reviews;
use merge_review::merge_review;
use merge_review_diff::merge_review_diff;
use publish_review::publish_review;
use publish_review_diff::publish_review_diff;
use remove_review_reviewer::remove_review_reviewer;
use resolve_review_comment::resolve_review_comment;
use update_review::update_review;
use update_review_comment::update_review_comment;
use update_review_diff::update_review_diff;

pub fn create_review_router() -> Router<AppState> {
    Router::new()
        .route(
            "/repository/{owner}/{repo}/review/{number}",
            get(get_review).patch(update_review),
        )
        .route("/repository/{owner}/{repo}/reviews", get(list_reviews))
        .route(
            "/repository/{owner}/{repo}/review/{number}/publish",
            post(publish_review),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/reviewer",
            post(add_review_reviewer),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/reviewer/{reviewer_name}",
            delete(remove_review_reviewer),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}",
            get(get_review_diff).patch(update_review_diff),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/merge",
            post(merge_review),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}/merge",
            post(merge_review_diff),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}/publish",
            post(publish_review_diff),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}/approve",
            post(approve_review_diff),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}/reject",
            post(reject_review_diff),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}/comments",
            post(create_review_comments),
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
