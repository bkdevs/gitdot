use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct DeleteWebhook;

impl Endpoint for DeleteWebhook {
    const PATH: &'static str = "/repository/{owner}/{repo}/webhook/{webhook_id}";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = DeleteWebhookRequest;
    type Response = DeleteWebhookResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct DeleteWebhookRequest {}

pub type DeleteWebhookResponse = ();
