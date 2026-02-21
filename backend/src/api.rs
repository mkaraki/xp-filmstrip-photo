use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;
use crate::validate_path;
use crate::image_handler;
use image::GenericImageView;

#[derive(Serialize, Deserialize, Clone)]
pub struct FolderItem {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub mime: Option<String>,
    pub has_subdirs: bool,
    pub size: u64,
    pub modified: u64,
}

#[derive(Serialize)]
pub struct FileMetadata {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize)]
pub struct VersionInfo {
    pub version: &'static str,
    pub build_time: &'static str,
}

pub fn router(_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/list/{*path}", get(list_folder))
        .route("/dirs/{*path}", get(list_dirs))
        .route("/metadata/{*path}", get(get_metadata))
        .route("/version.json", get(get_version))
        .route("/thumb/{*path}", get(image_handler::get_thumbnail))
        
        .route("/list.json", get(list_root_folder))
        .route("/dirs.json", get(list_root_dirs))
        .route("/thumb", get(image_handler::get_thumbnail_root))
}

async fn list_root_folder(state: State<Arc<AppState>>) -> Result<Json<Vec<FolderItem>>, axum::http::StatusCode> {
    list_folder(state, Path("".to_string())).await
}

async fn list_root_dirs(state: State<Arc<AppState>>) -> Result<Json<Vec<FolderItem>>, axum::http::StatusCode> {
    list_dirs(state, Path("".to_string())).await
}

async fn has_subdirs(full_path: &std::path::Path) -> bool {
    if let Ok(mut dir) = tokio::fs::read_dir(full_path).await {
        while let Ok(Some(entry)) = dir.next_entry().await {
            if let Ok(meta) = entry.metadata().await {
                if meta.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !name.starts_with('.') {
                        return true;
                    }
                }
            }
        }
    }
    false
}

async fn list_folder(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<Vec<FolderItem>>, axum::http::StatusCode> {
    let path = if path.ends_with(".json") {
        &path[..path.len() - 5]
    } else if !path.is_empty() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    } else {
        &path
    };

    let full_path = validate_path(&state.photo_root, path)
        .map_err(|_| axum::http::StatusCode::FORBIDDEN)?;

    if !full_path.is_dir() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    let mut items = Vec::new();
    let mut dir = tokio::fs::read_dir(&full_path).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(entry) = dir.next_entry().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)? {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata().await.ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = metadata.as_ref()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let mut rel_path = path.trim_start_matches('/').to_string();
        if !rel_path.is_empty() {
            if !rel_path.ends_with('/') {
                rel_path.push('/');
            }
        }
        rel_path.push_str(&name);

        let mut item_has_subdirs = false;
        if is_dir {
            item_has_subdirs = has_subdirs(&entry.path()).await;
        }

        let mime = if !is_dir {
            Some(mime_guess::from_path(entry.path()).first_or_octet_stream().to_string())
        } else {
            None
        };

        items.push(FolderItem {
            name,
            path: rel_path,
            is_dir,
            mime,
            has_subdirs: item_has_subdirs,
            size,
            modified,
        });
    }

    items.sort_by(|a, b| {
        if a.is_dir == b.is_dir {
            a.name.cmp(&b.name)
        } else if a.is_dir {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    Ok(Json(items))
}

async fn list_dirs(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<Vec<FolderItem>>, axum::http::StatusCode> {
    let path = if path.ends_with(".json") {
        &path[..path.len() - 5]
    } else if !path.is_empty() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    } else {
        &path
    };

    let full_path = validate_path(&state.photo_root, path)
        .map_err(|_| axum::http::StatusCode::FORBIDDEN)?;

    if !full_path.is_dir() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    let mut items = Vec::new();
    let mut dir = tokio::fs::read_dir(&full_path).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(entry) = dir.next_entry().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)? {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata().await.ok();
        if metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false) {
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            let modified = metadata.as_ref()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let mut rel_path = path.trim_start_matches('/').to_string();
            if !rel_path.is_empty() {
                if !rel_path.ends_with('/') {
                    rel_path.push('/');
                }
            }
            rel_path.push_str(&name);

            items.push(FolderItem {
                name,
                path: rel_path,
                is_dir: true,
                mime: None,
                has_subdirs: has_subdirs(&entry.path()).await,
                size,
                modified,
            });
        }
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Json(items))
}

async fn get_metadata(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<FileMetadata>, axum::http::StatusCode> {
    let path = if path.ends_with(".json") {
        &path[..path.len() - 5]
    } else {
        return Err(axum::http::StatusCode::NOT_FOUND);
    };

    let full_path = validate_path(&state.photo_root, path)
        .map_err(|_| axum::http::StatusCode::FORBIDDEN)?;

    if !full_path.is_file() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    let metadata = tokio::task::spawn_blocking(move || {
        image::open(full_path).map(|img| {
            let (width, height) = img.dimensions();
            FileMetadata { width, height }
        })
    }).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
      .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    Ok(Json(metadata))
}

async fn get_version() -> Json<VersionInfo> {
    Json(VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        build_time: env!("BUILD_TIME"),
    })
}
