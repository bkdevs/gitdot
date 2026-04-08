use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode, header},
};
use base64::prelude::*;
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn upload_user_image(
    auth_user: Principal<User>,
    State(_state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<AppResponse<()>, AppError> {
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !matches!(content_type, "image/jpeg" | "image/png" | "image/webp") {
        return Err(anyhow::anyhow!("unsupported image type: {content_type}").into());
    }

    let bytes = body.to_vec();
    let webp_bytes = tokio::task::spawn_blocking(move || -> anyhow::Result<Vec<u8>> {
        let img = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()?
            .decode()?;

        let img = img.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3);
        let mut out = Cursor::new(Vec::new());
        img.write_to(&mut out, ImageFormat::WebP)?;
        Ok(out.into_inner())
    })
    .await
    .map_err(|e| anyhow::anyhow!("spawn error: {e}"))??;

    // TODO: persist webp_bytes + update user avatar_url once repo layer is ready
    let b64 = BASE64_STANDARD.encode(&webp_bytes);
    tracing::info!("webp ({} bytes): data:image/webp;base64,{}", webp_bytes.len(), b64);
    let _ = auth_user.id;

    Ok(AppResponse::new(StatusCode::OK, ()))
}
