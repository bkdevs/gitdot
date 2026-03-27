use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::WebhookResource};

pub struct ListWebhooks;

impl Endpoint for ListWebhooks {
    const PATH: &'static str = "/repository/{owner}/{repo}/webhooks";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListWebhooksRequest;
    type Response = ListWebhooksResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListWebhooksRequest;

pub type ListWebhooksResponse = Vec<WebhookResource>;
