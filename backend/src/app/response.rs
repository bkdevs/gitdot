use api::ApiResource;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Clone)]
pub struct AppResponse<T: ApiResource>(StatusCode, T);

impl<T: ApiResource> AppResponse<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self(status_code, data)
    }
}

impl<T: ApiResource> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        (self.0, Json(self.1)).into_response()
    }
}
