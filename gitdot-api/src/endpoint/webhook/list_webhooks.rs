use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{WebhookResource, common::Page},
};

pub struct ListWebhooks;

impl Endpoint for ListWebhooks {
    const PATH: &'static str = "/repository/{owner}/{repo}/webhooks";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListWebhooksRequest;
    type Response = ListWebhooksResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListWebhooksRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListWebhooksResponse = Page<WebhookResource>;
