use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse},
};
use std::sync::Arc;
use crate::AppState;
use crate::validate_path;
use image::GenericImageView;
use std::io::Cursor;
use fast_image_resize as fr;
use sha2::{Sha256, Digest};
use std::path::PathBuf;
use tokio::fs;
use httpdate::HttpDate;

pub async fn get_thumbnail_root(
    state: State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    get_thumbnail(state, headers, Path("".to_string())).await
}

pub async fn get_thumbnail(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let full_path = match validate_path(&state.photo_root, &path) {
        Ok(p) => p,
        Err(_) => return StatusCode::FORBIDDEN.into_response(),
    };

    if !full_path.is_file() {
        return StatusCode::NOT_FOUND.into_response();
    }

    // Get original file metadata
    let metadata = match fs::metadata(&full_path).await {
        Ok(m) => m,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    
    let modified_time = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
    let last_modified = HttpDate::from(modified_time).to_string();

    // Check If-Modified-Since
    if let Some(if_modified_since) = headers.get(header::IF_MODIFIED_SINCE) {
        if if_modified_since == last_modified.as_str() {
            return StatusCode::NOT_MODIFIED.into_response();
        }
    }

    let modified_secs = modified_time.duration_since(std::time::UNIX_EPOCH).ok()
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Disk Cache logic
    let mut hasher = Sha256::new();
    hasher.update(full_path.to_string_lossy().as_bytes());
    hasher.update(modified_secs.to_le_bytes());
    let cache_key = format!("{:x}.jpg", hasher.finalize());
    
    let cache_dir = PathBuf::from(".cache/thumbs");
    let cache_path = cache_dir.join(&cache_key);

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).await.ok();
    }

    if let Ok(cached_data) = fs::read(&cache_path).await {
        return (
            [
                (header::CONTENT_TYPE, "image/jpeg".to_string()),
                (header::LAST_MODIFIED, last_modified),
                (header::CACHE_CONTROL, "public, max-age=31536000".to_string()),
            ],
            cached_data,
        ).into_response();
    }

    // Fallback: Generate thumbnail
    let mime = mime_guess::from_path(&full_path).first_or_octet_stream();
    if !mime.type_().to_string().starts_with("image") {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let img_data = match fs::read(&full_path).await {
        Ok(d) => d,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let img = match image::load_from_memory(&img_data) {
        Ok(i) => i,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let (width, height) = img.dimensions();
    let max_thumb_size = 200.0;
    let ratio = (max_thumb_size / width as f32).min(max_thumb_size / height as f32);
    
    let target_width = (width as f32 * ratio).round() as u32;
    let target_height = (height as f32 * ratio).round() as u32;
    let target_width = target_width.max(1);
    let target_height = target_height.max(1);

    let src_image = fr::images::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    ).unwrap();

    let mut dst_image = fr::images::Image::new(
        target_width,
        target_height,
        fr::PixelType::U8x4,
    );

    let mut resizer = fr::Resizer::new();
    resizer.resize(&src_image, &mut dst_image, &fr::ResizeOptions::default()).unwrap();

    let mut buffer = Cursor::new(Vec::new());
    let dynamic_img = image::DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(target_width, target_height, dst_image.into_vec()).unwrap()
    );
    
    match dynamic_img.write_to(&mut buffer, image::ImageFormat::Jpeg) {
        Ok(_) => {
            let data = buffer.into_inner();
            let _ = fs::write(&cache_path, &data).await;
            
            (
                [
                    (header::CONTENT_TYPE, "image/jpeg".to_string()),
                    (header::LAST_MODIFIED, last_modified),
                    (header::CACHE_CONTROL, "public, max-age=31536000".to_string()),
                ],
                data,
            ).into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
