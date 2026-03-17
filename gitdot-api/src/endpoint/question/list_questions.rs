use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::question::QuestionResource};

pub struct ListQuestions;

impl Endpoint for ListQuestions {
    const PATH: &'static str = "/repository/{owner}/{repo}/questions";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListQuestionsRequest;
    type Response = ListQuestionsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListQuestionsRequest;

pub type ListQuestionsResponse = Vec<QuestionResource>;
