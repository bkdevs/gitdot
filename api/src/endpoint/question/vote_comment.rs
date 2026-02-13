use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::question::VoteResource};

pub struct VoteComment;

impl Endpoint for VoteComment {
    const PATH: &'static str =
        "/repository/{owner}/{repo}/question/{number}/comment/{comment_id}/vote";
    const METHOD: http::Method = http::Method::POST;

    type Request = VoteCommentRequest;
    type Response = VoteCommentResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteCommentRequest {
    pub value: i16,
}

pub type VoteCommentResponse = VoteResource;
