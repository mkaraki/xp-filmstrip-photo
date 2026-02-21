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

#[derive(Serialize, Deserialize)]
pub struct FolderItem {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub mime: Option<String>,
}

pub fn router(_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        // Wildcard routes for subpaths (must come AFTER more specific routes or be carefully handled)
        .route("/list/{*path}", get(list_folder))
        .route("/dirs/{*path}", get(list_dirs))
        .route("/thumb/{*path}", get(image_handler::get_thumbnail))
        
        // Explicit routes for root and trailing slashes
        .route("/list/", get(list_root_folder))
        .route("/list", get(list_root_folder))
        .route("/dirs/", get(list_root_dirs))
        .route("/dirs", get(list_root_dirs))
        .route("/thumb/", get(image_handler::get_thumbnail_root))
        .route("/thumb", get(image_handler::get_thumbnail_root))
}

async fn list_root_folder(state: State<Arc<AppState>>) -> Result<Json<Vec<FolderItem>>, axum::http::StatusCode> {
    list_folder(state, Path("".to_string())).await
}

async fn list_root_dirs(state: State<Arc<AppState>>) -> Result<Json<Vec<FolderItem>>, axum::http::StatusCode> {
    list_dirs(state, Path("".to_string())).await
}

async fn list_folder(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> Result<Json<Vec<FolderItem>>, axum::http::StatusCode> {
    let full_path = validate_path(&state.photo_root, &path)
        .map_err(|_| axum::http::StatusCode::FORBIDDEN)?;

    if !full_path.is_dir() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    let mut items = Vec::new();
    let mut dir = tokio::fs::read_dir(full_path).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(entry) = dir.next_entry().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)? {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata().await.ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        
        let mut rel_path = path.trim_start_matches('/').to_string();
        if !rel_path.is_empty() {
            if !rel_path.ends_with('/') {
                rel_path.push('/');
            }
        }
        rel_path.push_str(&name);

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
    let full_path = validate_path(&state.photo_root, &path)
        .map_err(|_| axum::http::StatusCode::FORBIDDEN)?;

    if !full_path.is_dir() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    let mut items = Vec::new();
    let mut dir = tokio::fs::read_dir(full_path).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(entry) = dir.next_entry().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)? {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata().await.ok();
        if metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false) {
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
            });
        }
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Json(items))
}
