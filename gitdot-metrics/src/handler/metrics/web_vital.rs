use axum::{Json, extract::State, http::StatusCode};

use gitdot_api::endpoint::metrics::web_vital as api;
use gitdot_axum::extract::{ClientIp, Principal, UserAgent};
use gitdot_core::dto::{LogWebVitalRequest, WebVitalEventInput};

use crate::app::{AppError, AppState};

pub async fn log_web_vital(
    principal: Option<Principal>,
    ClientIp(ip): ClientIp,
    UserAgent(user_agent): UserAgent,
    State(state): State<AppState>,
    Json(body): Json<api::LogWebVitalRequest>,
) -> Result<StatusCode, AppError> {
    let events = body
        .events
        .into_iter()
        .map(|e| WebVitalEventInput {
            event_time_ms: e.event_time,
            name: e.name,
            value: e.value,
            rating: e.rating,
            metric_id: e.metric_id,
            navigation_type: e.navigation_type,
            route: e.route,
            path: e.path,
        })
        .collect();
    let request = LogWebVitalRequest::new(
        events,
        principal.map(|p| p.id),
        ip.unwrap_or_default(),
        user_agent.unwrap_or_default(),
        body.country.unwrap_or_default(),
        body.region.unwrap_or_default(),
        body.city.unwrap_or_default(),
    )?;
    state.metrics_service.log_web_vital(request).await?;

    Ok(StatusCode::NO_CONTENT)
}
