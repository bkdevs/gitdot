use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::review::ReviewResource};

pub struct PublishReviewDiff;

impl Endpoint for PublishReviewDiff {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/review/{number}/diff/{position}/publish";
    const METHOD: http::Method = http::Method::POST;

    type Request = PublishReviewDiffRequest;
    type Response = PublishReviewDiffResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct PublishReviewDiffRequest {}

pub type PublishReviewDiffResponse = ReviewResource;
