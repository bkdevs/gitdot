use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use gitdot_api::ApiResource;

#[derive(Debug, Clone)]
pub enum AppResponse<T: ApiResource> {
    Data(StatusCode, T),
    NotModified,
}

impl<T: ApiResource> AppResponse<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self::Data(status_code, data)
    }
}

impl<T: ApiResource> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        match self {
            AppResponse::Data(status, data) => (status, Json(data)).into_response(),
            AppResponse::NotModified => StatusCode::NOT_MODIFIED.into_response(),
        }
    }
}
