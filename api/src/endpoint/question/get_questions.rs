use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::question::QuestionResource};

pub struct GetQuestions;

impl Endpoint for GetQuestions {
    const PATH: &'static str = "/repository/{owner}/{repo}/questions";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetQuestionsRequest;
    type Response = GetQuestionsResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetQuestionsRequest;

pub type GetQuestionsResponse = Vec<QuestionResource>;
