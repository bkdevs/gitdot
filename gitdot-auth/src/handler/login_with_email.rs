use axum::{Json, http::StatusCode};

use crate::dto::LoginWithEmailRequest;

pub async fn login_with_email(Json(_request): Json<LoginWithEmailRequest>) -> StatusCode {
    StatusCode::OK
}
