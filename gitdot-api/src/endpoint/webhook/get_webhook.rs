use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::WebhookResource};

pub struct GetWebhook;

impl Endpoint for GetWebhook {
    const PATH: &'static str = "/repository/{owner}/{repo}/webhook/{webhook_id}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetWebhookRequest;
    type Response = GetWebhookResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetWebhookRequest;

pub type GetWebhookResponse = WebhookResource;
