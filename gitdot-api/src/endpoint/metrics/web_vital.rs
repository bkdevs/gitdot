use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct LogWebVital;

impl Endpoint for LogWebVital {
    const PATH: &'static str = "/metrics/web-vital";
    const METHOD: http::Method = http::Method::POST;

    type Request = LogWebVitalRequest;
    type Response = ();
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct LogWebVitalRequest {
    pub events: Vec<WebVitalEvent>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebVitalEvent {
    pub event_time: i64,
    pub name: String,
    pub value: f64,
    pub rating: String,
    pub metric_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigation_type: Option<String>,
    pub route: String,
    pub path: String,
}
