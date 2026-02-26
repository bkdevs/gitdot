use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use futures::TryStreamExt;
use serde::Deserialize;

use gitdot_core::dto::{GitHttpBody, GitHttpResponse};

#[derive(Deserialize)]
pub struct InfoRefsQuery {
    pub service: String,
}

pub struct GitHttpServerResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: GitHttpBody,
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

        let body = match self.body {
            GitHttpBody::Buffered(bytes) => Body::from(bytes),
            GitHttpBody::Stream(s) => {
                Body::from_stream(s.map_ok(|chunk| axum::body::Bytes::from(chunk)))
            }
        };

        builder.body(body).unwrap_or_else(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        })
    }
}
