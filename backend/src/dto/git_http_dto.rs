use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use gitdot_core::dto::GitHttpResponse;

pub struct GitHttpAxumResponse(pub GitHttpResponse);

impl From<GitHttpResponse> for GitHttpAxumResponse {
    fn from(response: GitHttpResponse) -> Self {
        Self(response)
    }
}

impl IntoResponse for GitHttpAxumResponse {
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
