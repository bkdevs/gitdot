use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode, header},
};

use gitdot_core::{
    dto::UpdateCurrentUserImageRequest,
    error::{ImageError, UserError},
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn upload_user_image(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<AppResponse<()>, AppError> {
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !matches!(content_type, "image/jpeg" | "image/png" | "image/webp") {
        return Err(UserError::InvalidImage(ImageError::DecodeError(format!(
            "unsupported image type: {content_type}"
        )))
        .into());
    }

    state
        .user_service
        .update_current_user_image(UpdateCurrentUserImageRequest::new(auth_user.id, body))
        .await?;

    Ok(AppResponse::new(StatusCode::NO_CONTENT, ()))
}
