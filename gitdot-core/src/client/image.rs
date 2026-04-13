use std::io::Cursor;

use async_trait::async_trait;
use bytes::Bytes;
use image::{ImageFormat, ImageReader};

use crate::{error::ImageError, util::image::beam_svg};

#[async_trait]
pub trait ImageClient: Send + Sync + Clone + 'static {
    async fn convert_to_webp(&self, bytes: Bytes) -> Result<Bytes, ImageError>;
    async fn generate_user_image(&self, email: &str) -> Result<Bytes, ImageError>;
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

    async fn generate_user_image(&self, email: &str) -> Result<Bytes, ImageError> {
        let email = email.to_string();
        let bytes = tokio::task::spawn_blocking(move || {
            let svg = beam_svg(&email);
            let opt = resvg::usvg::Options::default();
            let tree = resvg::usvg::Tree::from_str(&svg, &opt)
                .map_err(|e| ImageError::GenerateError(e.to_string()))?;

            let mut pixmap = tiny_skia::Pixmap::new(64, 64)
                .ok_or_else(|| ImageError::GenerateError("pixmap alloc failed".into()))?;
            resvg::render(
                &tree,
                tiny_skia::Transform::identity(),
                &mut pixmap.as_mut(),
            );

            // encode_png handles premultiplied→straight alpha (needed for anti-aliased circle edges)
            let png = pixmap
                .encode_png()
                .map_err(|e| ImageError::GenerateError(e.to_string()))?;

            let img = image::load_from_memory_with_format(&png, ImageFormat::Png)
                .map_err(|e| ImageError::GenerateError(e.to_string()))?;

            let mut out = Cursor::new(Vec::new());
            img.write_to(&mut out, ImageFormat::WebP)
                .map_err(|e| ImageError::EncodeError(e.to_string()))?;

            Ok::<Vec<u8>, ImageError>(out.into_inner())
        })
        .await
        .map_err(|_| ImageError::SpawnError)??;
        Ok(Bytes::from(bytes))
    }
}
