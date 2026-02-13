use crate::endpoint::Endpoint;
use crate::resource::question::VoteResource;
use serde::{Deserialize, Serialize};

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
