use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use gitdot_api::ApiResource;

#[derive(Debug, Clone)]
pub struct AppResponse<T: ApiResource>(StatusCode, T);

impl<T: ApiResource> AppResponse<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        Self(status, data)
    }
}

impl<T: ApiResource> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        if matches!(self.0, StatusCode::NO_CONTENT | StatusCode::NOT_MODIFIED) {
            return self.0.into_response();
        }
        (self.0, Json(self.1)).into_response()
    }
}
