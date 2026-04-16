use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct PublishReview;

impl Endpoint for PublishReview {
    const PATH: &'static str = "/repository/{owner}/{repo}/review/{number}/publish";
    const METHOD: http::Method = http::Method::POST;

    type Request = PublishReviewRequest;
    type Response = ();
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct PublishReviewRequest {}
