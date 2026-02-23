use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use base64::{engine::general_purpose::{self, STANDARD, URL_SAFE}, Engine as _};
use std::fs;
use std::path::Path;
use std::sync::RwLock;
use std::time::SystemTime;
use std::sync::Arc;

pub struct AuthState {
    pub htpasswd_path: String,
    users: RwLock<Vec<(String, String)>>,
    last_modified: RwLock<Option<SystemTime>>,
}

impl AuthState {
    pub fn new(htpasswd_path: String) -> Self {
        let state = Self {
            htpasswd_path,
            users: RwLock::new(Vec::new()),
            last_modified: RwLock::new(None),
        };
        state.reload_if_needed();
        state
    }

    pub fn is_enabled(&self) -> bool {
        Path::new(&self.htpasswd_path).exists()
    }

    fn reload_if_needed(&self) {
        let path = Path::new(&self.htpasswd_path);
        if !path.exists() {
            return;
        }

        let metadata = fs::metadata(path).ok();
        let modified = metadata.and_then(|m| m.modified().ok());

        let mut last_mod = self.last_modified.write().unwrap();
        if *last_mod != modified {
            if let Ok(content) = fs::read_to_string(path) {
                let mut users = self.users.write().unwrap();
                users.clear();
                for line in content.lines() {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        users.push((parts[0].to_string(), parts[1].to_string()));
                    }
                }
                *last_mod = modified;
            }
        }
    }

    pub fn verify(&self, auth_header: &str) -> bool {
        self.reload_if_needed();

        let credentials_b64 = if auth_header.to_lowercase().starts_with("basic ") {
            auth_header[6..].trim()
        } else {
            auth_header.trim()
        };

        // Standard base64 decode
        let mut decode_res = STANDARD.decode(credentials_b64);
        
        // If it fails, try URL safe as a fallback just in case
        if decode_res.is_err() {
            decode_res = URL_SAFE.decode(credentials_b64);
        }

        let Ok(credentials) = decode_res else {
            return false;
        };
        
        let Ok(credentials_str) = String::from_utf8(credentials) else {
            return false;
        };

        let parts: Vec<&str> = credentials_str.splitn(2, ':').collect();
        if parts.len() != 2 {
            return false;
        }

        let (username, password) = (parts[0], parts[1]);
        let users = self.users.read().unwrap();

        for (u, h) in users.iter() {
            if u == username {
                if h.starts_with("$2y$") || h.starts_with("$2b$") || h.starts_with("$2a$") {
                    if bcrypt::verify(password, h).unwrap_or(false) {
                        return true;
                    }
                } else if h == password {
                    return true;
                }
            }
        }

        false
    }
}

pub async fn basic_auth_middleware(
    State(state): State<Arc<AuthState>>,
    req: Request,
    next: Next,
) -> Response {
    if !state.is_enabled() {
        return next.run(req).await;
    }

    if req.method() == axum::http::Method::OPTIONS {
        return next.run(req).await;
    }
    
    let path = req.uri().path();
    // Allow login and version info to pass through the middleware.
    // login_handler will perform its own verification.
    if path == "/.__api/version.json" || path == "/.__api/auth/login" {
        return next.run(req).await;
    }

    let mut authenticated = false;
    
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION).and_then(|h| h.to_str().ok()) {
        if state.verify(auth_header) {
            authenticated = true;
        }
    }

    if !authenticated {
        if let Some(cookie) = req.headers().get(header::COOKIE).and_then(|h| h.to_str().ok()) {
            for part in cookie.split(';') {
                let part = part.trim();
                if part.starts_with("filmstrip_auth=") {
                    let creds = &part["filmstrip_auth=".len()..];
                    if state.verify(creds) {
                        authenticated = true;
                        break;
                    }
                }
            }
        }
    }

    if authenticated {
        next.run(req).await
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

pub async fn login_handler(
    State(state): State<Arc<crate::AppState>>,
    req: Request,
) -> impl IntoResponse {
    let auth_header = req.headers().get(header::AUTHORIZATION).and_then(|h| h.to_str().ok());
    
    match auth_header {
        Some(auth) if state.auth.verify(auth) => {
            let credentials_b64 = if auth.starts_with("Basic ") {
                &auth[6..]
            } else {
                auth
            };
            
            let cookie = format!("filmstrip_auth={}; Path=/; SameSite=Lax; Max-Age=31536000", credentials_b64);
            (
                StatusCode::OK,
                [(header::SET_COOKIE, cookie)],
                Json(serde_json::json!({ "status": "ok" }))
            ).into_response()
        }
        _ => StatusCode::UNAUTHORIZED.into_response()
    }
}

pub async fn logout_handler() -> impl IntoResponse {
    let cookie = "filmstrip_auth=; Path=/; SameSite=Lax; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT";
    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(serde_json::json!({ "status": "ok" }))
    ).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Helper function to create a temporary htpasswd file
    fn create_temp_htpasswd(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();
        file
    }

    #[test]
    fn test_authstate_new() {
        let state = AuthState::new("nonexistent.htpasswd".to_string());
        assert_eq!(state.htpasswd_path, "nonexistent.htpasswd");
    }

    #[test]
    fn test_is_enabled_when_file_exists() {
        let file = create_temp_htpasswd("user:password\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);
        assert!(state.is_enabled());
    }

    #[test]
    fn test_is_enabled_when_file_not_exists() {
        let state = AuthState::new("nonexistent.htpasswd".to_string());
        assert!(!state.is_enabled());
    }

    #[test]
    fn test_verify_with_plain_text_password() {
        let file = create_temp_htpasswd("testuser:testpass\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        // Create Basic Auth header: base64("testuser:testpass")
        let credentials = STANDARD.encode("testuser:testpass");
        let auth_header = format!("Basic {}", credentials);

        assert!(state.verify(&auth_header));
    }

    #[test]
    fn test_verify_with_plain_text_password_without_basic_prefix() {
        let file = create_temp_htpasswd("testuser:testpass\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        // Just the base64 encoded credentials without "Basic " prefix
        let credentials = STANDARD.encode("testuser:testpass");

        assert!(state.verify(&credentials));
    }

    #[test]
    fn test_verify_with_bcrypt_password() {
        // bcrypt hash for "testpass" with cost 4 (lower cost for faster tests)
        let bcrypt_hash = bcrypt::hash("testpass", 4).unwrap();
        let content = format!("testuser:{}\n", bcrypt_hash);
        let file = create_temp_htpasswd(&content);
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        let credentials = STANDARD.encode("testuser:testpass");
        let auth_header = format!("Basic {}", credentials);

        assert!(state.verify(&auth_header));
    }

    #[test]
    fn test_verify_with_wrong_password() {
        let file = create_temp_htpasswd("testuser:correctpass\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        let credentials = STANDARD.encode("testuser:wrongpass");
        let auth_header = format!("Basic {}", credentials);

        assert!(!state.verify(&auth_header));
    }

    #[test]
    fn test_verify_with_wrong_username() {
        let file = create_temp_htpasswd("testuser:testpass\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        let credentials = STANDARD.encode("wronguser:testpass");
        let auth_header = format!("Basic {}", credentials);

        assert!(!state.verify(&auth_header));
    }

    #[test]
    fn test_verify_with_invalid_base64() {
        let file = create_temp_htpasswd("testuser:testpass\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        let auth_header = "Basic invalid!!!base64";

        assert!(!state.verify(auth_header));
    }

    #[test]
    fn test_verify_with_malformed_credentials() {
        let file = create_temp_htpasswd("testuser:testpass\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        // Credentials without colon separator
        let credentials = STANDARD.encode("testusernocolon");
        let auth_header = format!("Basic {}", credentials);

        assert!(!state.verify(&auth_header));
    }

    #[test]
    fn test_verify_with_multiple_users() {
        let file = create_temp_htpasswd("user1:pass1\nuser2:pass2\nuser3:pass3\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        // Verify each user
        let creds1 = STANDARD.encode("user1:pass1");
        assert!(state.verify(&format!("Basic {}", creds1)));

        let creds2 = STANDARD.encode("user2:pass2");
        assert!(state.verify(&format!("Basic {}", creds2)));

        let creds3 = STANDARD.encode("user3:pass3");
        assert!(state.verify(&format!("Basic {}", creds3)));
    }

    #[test]
    fn test_verify_with_empty_htpasswd() {
        let file = create_temp_htpasswd("");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        let credentials = STANDARD.encode("user:pass");
        let auth_header = format!("Basic {}", credentials);

        assert!(!state.verify(&auth_header));
    }

    #[test]
    fn test_verify_with_url_safe_base64() {
        let file = create_temp_htpasswd("testuser:testpass\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        // Use URL_SAFE encoding
        let credentials = URL_SAFE.encode("testuser:testpass");
        let auth_header = format!("Basic {}", credentials);

        assert!(state.verify(&auth_header));
    }

    #[test]
    fn test_verify_case_insensitive_basic_prefix() {
        let file = create_temp_htpasswd("testuser:testpass\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        let credentials = STANDARD.encode("testuser:testpass");
        
        // Test with different cases
        assert!(state.verify(&format!("basic {}", credentials)));
        assert!(state.verify(&format!("BASIC {}", credentials)));
        assert!(state.verify(&format!("Basic {}", credentials)));
    }

    #[test]
    fn test_reload_if_needed_with_nonexistent_file() {
        let state = AuthState::new("nonexistent.htpasswd".to_string());
        // This should not panic
        state.reload_if_needed();
        assert!(!state.is_enabled());
    }

    #[test]
    fn test_htpasswd_with_colons_in_password() {
        // Password containing colons should work with splitn(2, ':')
        let file = create_temp_htpasswd("testuser:pass:with:colons\n");
        let path = file.path().to_str().unwrap().to_string();
        let state = AuthState::new(path);

        let credentials = STANDARD.encode("testuser:pass:with:colons");
        let auth_header = format!("Basic {}", credentials);

        assert!(state.verify(&auth_header));
    }
}
