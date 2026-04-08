use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use image::{ImageFormat, ImageReader};
use std::io::Cursor;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

const MAX_IMAGE_BYTES: usize = 5 * 1024 * 1024; // 5 MB

#[axum::debug_handler]
pub async fn upload_user_image(
    auth_user: Principal<User>,
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<AppResponse<()>, AppError> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| anyhow::anyhow!("multipart error: {e}"))?
        .ok_or_else(|| anyhow::anyhow!("no file field in multipart body"))?;

    let _content_type = field
        .content_type()
        .unwrap_or("application/octet-stream")
        .to_string();

    let bytes = field
        .bytes()
        .await
        .map_err(|e| anyhow::anyhow!("failed to read field bytes: {e}"))?
        .to_vec();

    if bytes.len() > MAX_IMAGE_BYTES {
        return Err(anyhow::anyhow!("image too large (max 5 MB)").into());
    }

    // Convert to WebP off the async runtime (CPU-bound)
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
