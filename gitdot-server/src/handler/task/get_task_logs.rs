use std::convert::Infallible;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures::StreamExt;
use uuid::Uuid;

use gitdot_core::error::TaskError;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_task_logs(
    State(state): State<AppState>,
    _auth: Principal<User>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let task = state
        .task_service
        .get_task(id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::Task(TaskError::NotFound(id.to_string())))?;

    let is_sse = headers
        .get("accept")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("text/event-stream"))
        .unwrap_or(false);

    if is_sse {
        let s2_stream = state
            .s2_client
            .tail_task_logs(&task.s2_uri, None)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        let sse_stream = s2_stream.map(|result| match result {
            Ok(records) => {
                let api_records = records.into_api();
                let last_seq = api_records.last().map(|r| r.seq_num);
                let data = serde_json::to_string(&api_records).unwrap_or_default();
                let event = Event::default().event("batch").data(data);
                let event = match last_seq {
                    Some(seq) => event.id(seq.to_string()),
                    None => event,
                };
                Ok::<Event, Infallible>(event)
            }
            Err(e) => Ok(Event::default().event("error").data(e)),
        });

        Ok(Sse::new(sse_stream)
            .keep_alive(KeepAlive::default())
            .into_response())
    } else {
        let records = state
            .s2_client
            .get_task_logs(&task.s2_uri, 100, 100)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        Ok(AppResponse::new(StatusCode::OK, records.into_api()).into_response())
    }
}
