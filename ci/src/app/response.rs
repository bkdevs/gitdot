use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct AppResponse<T: Serialize + PartialEq>(StatusCode, T);

impl<T: Serialize + PartialEq> AppResponse<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self(status_code, data)
    }
}

impl<T: Serialize + PartialEq> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        (self.0, Json(self.1)).into_response()
    }
}
