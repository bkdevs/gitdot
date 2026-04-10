use std::io::Cursor;

use async_trait::async_trait;
use bytes::Bytes;
use image::{ImageFormat, ImageReader};

use crate::error::ImageError;

#[async_trait]
pub trait ImageClient: Send + Sync + Clone + 'static {
    async fn convert_to_webp(&self, bytes: Bytes) -> Result<Bytes, ImageError>;
}

#[derive(Clone)]
pub struct ImageClientImpl;

impl ImageClientImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ImageClient for ImageClientImpl {
    async fn convert_to_webp(&self, bytes: Bytes) -> Result<Bytes, ImageError> {
        let webp_bytes = tokio::task::spawn_blocking(move || {
            let img = ImageReader::new(Cursor::new(bytes.as_ref()))
                .with_guessed_format()
                .map_err(|e| ImageError::DecodeError(e.to_string()))?
                .decode()
                .map_err(|e| ImageError::DecodeError(e.to_string()))?;
            let img = img.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3);
            let mut out = Cursor::new(Vec::new());
            img.write_to(&mut out, ImageFormat::WebP)
                .map_err(|e| ImageError::EncodeError(e.to_string()))?;
            Ok::<Vec<u8>, ImageError>(out.into_inner())
        })
        .await
        .map_err(|_| ImageError::SpawnError)??;

        Ok(Bytes::from(webp_bytes))
    }
}
