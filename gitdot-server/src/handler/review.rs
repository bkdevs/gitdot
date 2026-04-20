mod add_review_reviewer;
mod create_review_comment;
mod get_review;
mod get_review_diff;
mod judge_review_diff;
mod list_reviews;
mod merge_review_diff;
mod publish_review;
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
use gitdot_core::{dto::ReviewId, error::ReviewError};

pub(crate) struct ReviewIdParam(pub ReviewId);
impl<'de> serde::Deserialize<'de> for ReviewIdParam {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        if let Ok(n) = s.parse::<i32>() {
            return Ok(Self(ReviewId::Number(n)));
        }
        if s.len() == 8 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(Self(ReviewId::Hex(s.to_lowercase())));
        }
        Err(serde::de::Error::custom(ReviewError::InvalidIdentifier))
    }
}

use add_review_reviewer::add_review_reviewer;
use create_review_comment::create_review_comment;
use get_review::get_review;
use get_review_diff::get_review_diff;
use judge_review_diff::judge_review_diff;
use list_reviews::list_reviews;
use merge_review_diff::merge_review_diff;
use publish_review::publish_review;
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
            "/repository/{owner}/{repo}/review/{number}/diff/{position}/submit",
            post(judge_review_diff),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/diff/{position}/merge",
            post(merge_review_diff),
        )
        .route(
            "/repository/{owner}/{repo}/review/{number}/comment",
            post(create_review_comment),
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
