use axum::{
    Json,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use gitdot_api::ApiResource;

#[derive(Debug, Clone)]
pub struct AppResponse<T: ApiResource> {
    status: StatusCode,
    headers: HeaderMap,
    data: T,
}

impl<T: ApiResource> AppResponse<T> {
    pub fn new(status: StatusCode, data: T) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            data,
        }
    }

    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        if let (Ok(name), Ok(value)) = (name.parse::<HeaderName>(), value.parse::<HeaderValue>()) {
            self.headers.append(name, value);
        }
        self
    }
}

impl<T: ApiResource> IntoResponse for AppResponse<T> {
    fn into_response(self) -> Response {
        (self.status, self.headers, Json(self.data)).into_response()
    }
}
