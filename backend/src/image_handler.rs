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

    let metadata = match fs::metadata(&full_path).await {
        Ok(m) => m,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    
    let modified_time = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
    let (cache_path, last_modified) = get_cache_params(&full_path, modified_time);

    // Check If-Modified-Since
    if headers.get(header::IF_MODIFIED_SINCE).map_or(false, |v| v == last_modified.as_str()) {
        return StatusCode::NOT_MODIFIED.into_response();
    }

    if let Ok(cached_data) = fs::read(&cache_path).await {
        return (get_headers(&last_modified), cached_data).into_response();
    }

    match create_thumbnail_data(&full_path).await {
        Ok(data) => {
            let _ = save_to_cache(&cache_path, &data).await;
            (get_headers(&last_modified), data).into_response()
        }
        Err(status) => status.into_response(),
    }
}

fn get_cache_params(full_path: &std::path::Path, modified_time: std::time::SystemTime) -> (PathBuf, String) {
    let modified_secs = modified_time.duration_since(std::time::UNIX_EPOCH).ok()
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let mut hasher = Sha256::new();
    hasher.update(full_path.to_string_lossy().as_bytes());
    hasher.update(modified_secs.to_le_bytes());
    let cache_key = format!("{:x}.jpg", hasher.finalize());
    
    (PathBuf::from(".cache/thumbs").join(cache_key), HttpDate::from(modified_time).to_string())
}

fn get_headers(last_modified: &str) -> [(header::HeaderName, String); 3] {
    [
        (header::CONTENT_TYPE, "image/jpeg".to_string()),
        (header::LAST_MODIFIED, last_modified.to_string()),
        (header::CACHE_CONTROL, "public, max-age=31536000".to_string()),
    ]
}

async fn save_to_cache(cache_path: &PathBuf, data: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(cache_path, data).await
}

async fn create_thumbnail_data(full_path: &std::path::Path) -> Result<Vec<u8>, StatusCode> {
    let mime = mime_guess::from_path(full_path).first_or_octet_stream();
    if !mime.type_().to_string().starts_with("image") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let img_data = fs::read(full_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let img = image::load_from_memory(&img_data).map_err(|_| StatusCode::BAD_REQUEST)?;

    let (width, height) = img.dimensions();
    let max_thumb_size = 200.0;
    let ratio = (max_thumb_size / width as f32).min(max_thumb_size / height as f32);
    
    let target_width = ((width as f32 * ratio).round() as u32).max(1);
    let target_height = ((height as f32 * ratio).round() as u32).max(1);

    let src_image = fr::images::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut dst_image = fr::images::Image::new(target_width, target_height, fr::PixelType::U8x4);
    let mut resizer = fr::Resizer::new();
    resizer.resize(&src_image, &mut dst_image, &fr::ResizeOptions::default())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut buffer = Cursor::new(Vec::new());
    let dynamic_img = image::DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(target_width, target_height, dst_image.into_vec())
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
    );
    
    dynamic_img.write_to(&mut buffer, image::ImageFormat::Jpeg)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(buffer.into_inner())
}

