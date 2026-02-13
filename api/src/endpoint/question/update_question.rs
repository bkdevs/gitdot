use crate::endpoint::Endpoint;
use crate::resource::question::QuestionResource;
use serde::{Deserialize, Serialize};

pub struct UpdateQuestion;

impl Endpoint for UpdateQuestion {
    const PATH: &'static str = "/repository/{owner}/{repo}/question/{number}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateQuestionRequest;
    type Response = UpdateQuestionResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestionRequest {
    pub title: String,
    pub body: String,
}

pub type UpdateQuestionResponse = QuestionResource;
