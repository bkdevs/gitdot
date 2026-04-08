use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode, header},
};
use base64::prelude::*;
use gitdot_api::{
    endpoint::user::upload_user_image::UploadUserImageResponse,
    resource::user::UploadUserImageResource,
};
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

use gitdot_core::{dto::UpdateCurrentUserRequest, error::UserError};

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
) -> Result<AppResponse<UploadUserImageResponse>, AppError> {
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !matches!(content_type, "image/jpeg" | "image/png" | "image/webp") {
        return Err(
            UserError::InvalidImage(format!("unsupported image type: {content_type}")).into(),
        );
    }

    let bytes = body.to_vec();
    let webp_bytes = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, UserError> {
        let img = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()
            .map_err(|e| UserError::InvalidImage(e.to_string()))?
            .decode()
            .map_err(|e| UserError::InvalidImage(e.to_string()))?;

        let img = img.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3);
        let mut out = Cursor::new(Vec::new());
        img.write_to(&mut out, ImageFormat::WebP)
            .map_err(|e| UserError::InvalidImage(e.to_string()))?;
        Ok(out.into_inner())
    })
    .await
    .map_err(|_| UserError::InvalidImage("failed to convert to webp".to_string()))??;

    let b64 = BASE64_STANDARD.encode(&webp_bytes);
    state
        .user_service
        .update_current_user(UpdateCurrentUserRequest::with_image(
            auth_user.id,
            b64.clone(),
        ))
        .await?;

    Ok(AppResponse::new(
        StatusCode::OK,
        UploadUserImageResource { bytes: b64 },
    ))
}
