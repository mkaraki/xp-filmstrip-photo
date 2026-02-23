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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test_get_cache_params() {
        let path = PathBuf::from("/test/image.jpg");
        let modified_time = UNIX_EPOCH + std::time::Duration::from_secs(1234567890);
        
        let (cache_path, last_modified) = get_cache_params(&path, modified_time);
        
        // Check that cache path starts with .cache/thumbs
        assert!(cache_path.starts_with(".cache/thumbs"));
        
        // Check that cache path has .jpg extension
        assert_eq!(cache_path.extension().and_then(|s| s.to_str()), Some("jpg"));
        
        // Check that last_modified is not empty
        assert!(!last_modified.is_empty());
    }

    #[test]
    fn test_get_cache_params_different_paths_different_cache() {
        let path1 = PathBuf::from("/test/image1.jpg");
        let path2 = PathBuf::from("/test/image2.jpg");
        let modified_time = UNIX_EPOCH + std::time::Duration::from_secs(1234567890);
        
        let (cache_path1, _) = get_cache_params(&path1, modified_time);
        let (cache_path2, _) = get_cache_params(&path2, modified_time);
        
        // Different paths should have different cache files
        assert_ne!(cache_path1, cache_path2);
    }

    #[test]
    fn test_get_cache_params_different_times_different_cache() {
        let path = PathBuf::from("/test/image.jpg");
        let time1 = UNIX_EPOCH + std::time::Duration::from_secs(1000);
        let time2 = UNIX_EPOCH + std::time::Duration::from_secs(2000);
        
        let (cache_path1, last_modified1) = get_cache_params(&path, time1);
        let (cache_path2, last_modified2) = get_cache_params(&path, time2);
        
        // Same path but different modification times should have different cache files
        assert_ne!(cache_path1, cache_path2);
        assert_ne!(last_modified1, last_modified2);
    }

    #[test]
    fn test_get_cache_params_same_input_same_output() {
        let path = PathBuf::from("/test/image.jpg");
        let modified_time = UNIX_EPOCH + std::time::Duration::from_secs(1234567890);
        
        let (cache_path1, last_modified1) = get_cache_params(&path, modified_time);
        let (cache_path2, last_modified2) = get_cache_params(&path, modified_time);
        
        // Same inputs should produce same outputs (deterministic)
        assert_eq!(cache_path1, cache_path2);
        assert_eq!(last_modified1, last_modified2);
    }

    #[test]
    fn test_get_headers() {
        let last_modified = "Wed, 21 Oct 2015 07:28:00 GMT";
        let headers = get_headers(last_modified);
        
        assert_eq!(headers.len(), 3);
        
        // Check content type
        assert_eq!(headers[0].0, header::CONTENT_TYPE);
        assert_eq!(headers[0].1, "image/jpeg");
        
        // Check last modified
        assert_eq!(headers[1].0, header::LAST_MODIFIED);
        assert_eq!(headers[1].1, last_modified);
        
        // Check cache control
        assert_eq!(headers[2].0, header::CACHE_CONTROL);
        assert_eq!(headers[2].1, "public, max-age=31536000");
    }

    #[test]
    fn test_cache_path_uses_sha256() {
        let path = PathBuf::from("/test/image.jpg");
        let modified_time = UNIX_EPOCH + std::time::Duration::from_secs(1234567890);
        
        let (cache_path, _) = get_cache_params(&path, modified_time);
        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        
        // SHA256 produces 64 hex characters + .jpg extension
        assert_eq!(filename.len(), 68); // 64 + 4 for ".jpg"
        assert!(filename.ends_with(".jpg"));
        
        // Check that it's valid hex (all chars before .jpg should be 0-9a-f)
        let hex_part = &filename[..64];
        assert!(hex_part.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

