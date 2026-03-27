use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::WebhookResource};

pub struct CreateWebhook;

impl Endpoint for CreateWebhook {
    const PATH: &'static str = "/repository/{owner}/{repo}/webhook";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateWebhookRequest;
    type Response = CreateWebhookResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub secret: String,
    pub events: Vec<String>,
}

pub type CreateWebhookResponse = WebhookResource;
