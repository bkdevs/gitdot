use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

use crate::error::{InputError, MetricsError};

#[derive(Debug, Clone)]
pub struct LogWebVitalRequest {
    pub events: Vec<WebVitalEvent>,

    pub user_id: Option<Uuid>,
    pub ip: String,
    pub user_agent: String,

    pub country: String,
    pub region: String,
    pub city: String,
}

#[derive(Debug, Clone)]
pub struct WebVitalEvent {
    pub event_time: DateTime<Utc>,
    pub name: String,
    pub value: f64,
    pub rating: String,
    pub metric_id: String,
    pub navigation_type: Option<String>,
    pub route: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct WebVitalEventInput {
    pub event_time_ms: i64,
    pub name: String,
    pub value: f64,
    pub rating: String,
    pub metric_id: String,
    pub navigation_type: Option<String>,
    pub route: String,
    pub path: String,
}

impl LogWebVitalRequest {
    pub fn new(
        events: Vec<WebVitalEventInput>,
        user_id: Option<Uuid>,
        ip: String,
        user_agent: String,
        country: String,
        region: String,
        city: String,
    ) -> Result<Self, MetricsError> {
        let events = events
            .into_iter()
            .map(|e| {
                let event_time = Utc
                    .timestamp_millis_opt(e.event_time_ms)
                    .single()
                    .ok_or_else(|| InputError::new("event_time", "out of range"))?;
                Ok(WebVitalEvent {
                    event_time,
                    name: e.name,
                    value: e.value,
                    rating: e.rating,
                    metric_id: e.metric_id,
                    navigation_type: e.navigation_type,
                    route: e.route,
                    path: e.path,
                })
            })
            .collect::<Result<Vec<_>, MetricsError>>()?;

        Ok(Self {
            events,
            user_id,
            ip,
            user_agent,
            country,
            region,
            city,
        })
    }
}
