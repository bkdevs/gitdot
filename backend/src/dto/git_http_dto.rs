use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use gitdot_core::dto::GitHttpBackendResponse;

pub struct GitHttpResponse(pub GitHttpBackendResponse);

impl From<GitHttpBackendResponse> for GitHttpResponse {
    fn from(response: GitHttpBackendResponse) -> Self {
        Self(response)
    }
}

impl IntoResponse for GitHttpResponse {
    fn into_response(self) -> Response {
        let mut builder = Response::builder()
            .status(StatusCode::from_u16(self.0.status_code).unwrap_or(StatusCode::OK));

        for (name, value) in self.0.headers {
            builder = builder.header(name, value);
        }

        builder.body(Body::from(self.0.body)).unwrap_or_else(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        })
    }
}

#[derive(serde::Deserialize)]
pub struct InfoRefsQuery {
    pub service: String,
}
