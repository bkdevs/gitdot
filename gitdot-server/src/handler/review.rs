mod get_review;
mod get_reviews;

use axum::{Router, routing::get};

use crate::app::AppState;

use get_review::get_review;
use get_reviews::get_reviews;

pub fn create_review_router() -> Router<AppState> {
    Router::new()
        .route(
            "/repository/{owner}/{repo}/review/{number}",
            get(get_review),
        )
        .route("/repository/{owner}/{repo}/reviews", get(get_reviews))
}
