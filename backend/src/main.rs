use std::sync::Arc;
use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    response::{IntoResponse, Response},
    Router,
};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use std::path::{Path as StdPath, PathBuf};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use httpdate::HttpDate;

mod api;
mod image_handler;
mod auth;

#[derive(Clone)]
pub struct AppState {
    pub photo_root: PathBuf,
    pub auth: std::sync::Arc<auth::AuthState>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let photo_root = env::var("PHOTO_ROOT").unwrap_or_else(|_| "./fixtures".to_string());
    
    if !StdPath::new(&photo_root).exists() {
        std::fs::create_dir_all(&photo_root).ok();
    }
    
    let photo_root = PathBuf::from(photo_root).canonicalize().expect("Invalid PHOTO_ROOT");
    let htpasswd_path = env::var("HTPASSWD_PATH").unwrap_or_else(|_| ".htpasswd".to_string());

    let auth_state = std::sync::Arc::new(auth::AuthState::new(htpasswd_path));

    let state = Arc::new(AppState {
        photo_root,
        auth: auth_state.clone(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_router = api::router(state.clone())
        .layer(axum::middleware::from_fn_with_state(auth_state.clone(), auth::basic_auth_middleware));

    let app = Router::new()
        .nest("/.__api", api_router)
        .nest_service("/_nuxt", ServeDir::new("../frontend/.output/public/_nuxt"))
        .fallback(handle_path)
        .layer(cors)
        .with_state(state);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();

    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_path<B>(
    State(state): State<Arc<AppState>>,
    req: Request<B>,
) -> impl IntoResponse {
    let path_str = req.uri().path().trim_start_matches('/');
    let headers = req.headers();
    
    if path_str.starts_with(".__api") {
        return StatusCode::NOT_FOUND.into_response();
    }

    // Auth check if enabled, except for favicon, robots.txt and UI routes
    let is_public = path_str == "favicon.ico" || path_str == "robots.txt" || path_str.starts_with(".__ui/");
    if !is_public && state.auth.is_enabled() {
        let mut authenticated = false;
        
        if let Some(auth_header) = headers.get(header::AUTHORIZATION).and_then(|h| h.to_str().ok()) {
            if state.auth.verify(auth_header) {
                authenticated = true;
            }
        }

        if !authenticated {
            if let Some(cookie) = headers.get(header::COOKIE).and_then(|h| h.to_str().ok()) {
                for part in cookie.split(';') {
                    let part = part.trim();
                    if part.starts_with("filmstrip_auth=") {
                        let creds = &part["filmstrip_auth=".len()..];
                        if state.auth.verify(creds) {
                            authenticated = true;
                            break;
                        }
                    }
                }
            }
        }

        if !authenticated {
            // Check if it's a file request or something that needs auth
            let safe_path_res = validate_path(&state.photo_root, path_str);
            if let Ok(safe_path) = safe_path_res {
                if safe_path.is_file() && safe_path.starts_with(&state.photo_root) {
                    return StatusCode::UNAUTHORIZED.into_response();
                }
            }
        }
    }

    if path_str == "favicon.ico" || path_str == "robots.txt" {
         let static_path = PathBuf::from("../frontend/.output/public").join(path_str);
         if let Ok(content) = tokio::fs::read(&static_path).await {
             let mime = mime_guess::from_path(&static_path).first_or_octet_stream();
             return (
                 [
                     (header::CONTENT_TYPE, mime.to_string()),
                     (header::CACHE_CONTROL, "public, max-age=86400".to_string()),
                 ],
                 content,
             ).into_response();
         }
    }

    let safe_path = match validate_path(&state.photo_root, path_str) {
        Ok(p) => p,
        Err(_) => return StatusCode::FORBIDDEN.into_response(),
    };

    if safe_path.is_file() {
        match tokio::fs::metadata(&safe_path).await {
            Ok(metadata) => {
                let modified = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
                let last_modified = HttpDate::from(modified).to_string();

                if let Some(if_modified_since) = headers.get(header::IF_MODIFIED_SINCE) {
                    if if_modified_since == last_modified.as_str() {
                        return StatusCode::NOT_MODIFIED.into_response();
                    }
                }

                let mime = mime_guess::from_path(&safe_path).first_or_octet_stream();
                match tokio::fs::read(&safe_path).await {
                    Ok(content) => (
                        [
                            (header::CONTENT_TYPE, mime.to_string()),
                            (header::LAST_MODIFIED, last_modified),
                            (header::CACHE_CONTROL, "public, max-age=3600".to_string()),
                        ],
                        content,
                    ).into_response(),
                    Err(_) => StatusCode::NOT_FOUND.into_response(),
                }
            }
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        }
    } else {
        serve_nuxt_index().await
    }
}

async fn serve_nuxt_index() -> Response {
    let index_paths = [
        "../frontend/.output/public/index.html",
        "./frontend/.output/public/index.html",
        "./dist/index.html"
    ];

    for path in index_paths {
        if let Ok(content) = tokio::fs::read(path).await {
            return (
                [(header::CONTENT_TYPE, "text/html")],
                content,
            ).into_response();
        }
    }
    
    "Nuxt app not built. Run `bun run generate` in frontend directory.".into_response()
}

pub fn validate_path(root: &StdPath, sub_path: &str) -> Result<PathBuf, ()> {
    let sub_path = percent_encoding::percent_decode_str(sub_path)
        .decode_utf8()
        .map_err(|_| ())?;
    
    let mut joined = root.to_path_buf();
    for component in StdPath::new(sub_path.as_ref()).components() {
        match component {
            std::path::Component::Normal(c) => joined.push(c),
            std::path::Component::ParentDir => {
                if joined != root {
                    joined.pop();
                }
            },
            std::path::Component::RootDir | std::path::Component::CurDir => {},
            _ => return Err(()),
        }
    }

    if joined.starts_with(root) {
        Ok(joined)
    } else {
        Err(())
    }
}
