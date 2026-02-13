use crate::endpoint::Endpoint;
use crate::resource::question::QuestionResource;
use serde::{Deserialize, Serialize};

pub struct CreateQuestion;

impl Endpoint for CreateQuestion {
    const PATH: &'static str = "/repository/{owner}/{repo}/question";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateQuestionRequest;
    type Response = CreateQuestionResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestionRequest {
    pub title: String,
    pub body: String,
}

pub type CreateQuestionResponse = QuestionResource;
