use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use gitdot_core::dto::GitHttpResponse;

#[derive(Deserialize)]
pub struct InfoRefsQuery {
    pub service: String,
}

pub struct GitHttpServerResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl From<GitHttpResponse> for GitHttpServerResponse {
    fn from(response: GitHttpResponse) -> Self {
        Self {
            status_code: response.status_code,
            headers: response.headers,
            body: response.body,
        }
    }
}

impl IntoResponse for GitHttpServerResponse {
    fn into_response(self) -> Response {
        let mut builder = Response::builder()
            .status(StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::OK));

        for (name, value) in self.headers {
            builder = builder.header(name, value);
        }

        builder.body(Body::from(self.body)).unwrap_or_else(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        })
    }
}
