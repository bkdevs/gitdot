use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::WebhookResource};

pub struct UpdateWebhook;

impl Endpoint for UpdateWebhook {
    const PATH: &'static str = "/repository/{owner}/{repo}/webhook/{webhook_id}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateWebhookRequest;
    type Response = UpdateWebhookResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateWebhookRequest {
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
}

pub type UpdateWebhookResponse = WebhookResource;
