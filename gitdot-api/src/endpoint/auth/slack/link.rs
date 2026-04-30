use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::slack::SlackAccountResource};

pub struct LinkSlackAccount;

impl Endpoint for LinkSlackAccount {
    const PATH: &'static str = "/auth/slack/link";
    const METHOD: http::Method = http::Method::POST;

    type Request = LinkSlackAccountRequest;
    type Response = LinkSlackAccountResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct LinkSlackAccountRequest {
    pub state: String,
}

pub type LinkSlackAccountResponse = SlackAccountResource;
