use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::question::VoteResource};

pub struct VoteQuestion;

impl Endpoint for VoteQuestion {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}/vote";
    const METHOD: http::Method = http::Method::POST;

    type Request = VoteQuestionRequest;
    type Response = VoteQuestionResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteQuestionRequest {
    pub value: i16,
}

pub type VoteQuestionResponse = VoteResource;
