use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode, header},
};
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

        let mut out = Cursor::new(Vec::new());
        img.write_to(&mut out, ImageFormat::WebP)?;
        Ok(out.into_inner())
    })
    .await
    .map_err(|e| anyhow::anyhow!("spawn error: {e}"))??;

    // TODO: persist webp_bytes + update user avatar_url once repo layer is ready
    let _ = (auth_user.id, webp_bytes);

    Ok(AppResponse::new(StatusCode::OK, ()))
}
