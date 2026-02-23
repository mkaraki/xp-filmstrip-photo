# Backend Testing Documentation

## Overview
This document describes the test suite for the Filmstrip Photo backend written in Rust with Axum framework.

## Test Coverage

### Total Tests: 39

The test suite covers the following modules:

### 1. Authentication (`auth.rs`) - 16 tests
Tests for authentication and authorization functionality:

- **Basic Functionality**
  - `test_authstate_new()` - AuthState initialization
  - `test_is_enabled_when_file_exists()` - htpasswd file detection
  - `test_is_enabled_when_file_not_exists()` - missing htpasswd file

- **Authentication Verification**
  - `test_verify_with_plain_text_password()` - Plain text password with "Basic " prefix
  - `test_verify_with_plain_text_password_without_basic_prefix()` - Without prefix
  - `test_verify_with_bcrypt_password()` - Bcrypt hashed passwords
  - `test_verify_with_wrong_password()` - Failed auth with wrong password
  - `test_verify_with_wrong_username()` - Failed auth with wrong username

- **Edge Cases**
  - `test_verify_with_invalid_base64()` - Malformed base64
  - `test_verify_with_malformed_credentials()` - Credentials without colon separator
  - `test_verify_with_multiple_users()` - Multiple users in htpasswd
  - `test_verify_with_empty_htpasswd()` - Empty htpasswd file
  - `test_verify_with_url_safe_base64()` - URL-safe base64 encoding
  - `test_verify_case_insensitive_basic_prefix()` - Case-insensitive "Basic"
  - `test_htpasswd_with_colons_in_password()` - Passwords containing colons

- **Reload Functionality**
  - `test_reload_if_needed_with_nonexistent_file()` - Reload with missing file

### 2. API (`api.rs`) - 9 tests
Tests for API endpoints and data structures:

- **Path Normalization**
  - `test_normalize_api_path_with_json_extension()` - Normalizing .json paths
  - `test_normalize_api_path_empty()` - Empty path handling
  - `test_normalize_api_path_without_json_extension()` - Non-JSON extensions
  - `test_normalize_api_path_with_txt_extension()` - Invalid extensions

- **Data Structures**
  - `test_folder_item_creation()` - FolderItem struct creation
  - `test_folder_item_clone()` - FolderItem cloning
  - `test_file_metadata_creation()` - FileMetadata struct

- **Version Endpoint**
  - `test_get_version()` - Version info retrieval
  - `test_version_info_serialization()` - JSON serialization

### 3. Image Handler (`image_handler.rs`) - 6 tests
Tests for image thumbnail caching:

- **Cache Key Generation**
  - `test_get_cache_params()` - Basic cache parameter generation
  - `test_get_cache_params_different_paths_different_cache()` - Path uniqueness
  - `test_get_cache_params_different_times_different_cache()` - Time-based invalidation
  - `test_get_cache_params_same_input_same_output()` - Deterministic hashing
  - `test_cache_path_uses_sha256()` - SHA256 hash verification

- **HTTP Headers**
  - `test_get_headers()` - Response header generation

### 4. Path Validation (`main.rs`) - 8 tests
Tests for path security and validation:

- `test_validate_path_simple()` - Basic path validation
- `test_validate_path_empty()` - Empty path handling
- `test_validate_path_with_parent_dir()` - Parent directory traversal
- `test_validate_path_prevent_escape_at_root()` - Root escape prevention
- `test_validate_path_with_url_encoding()` - URL-encoded paths
- `test_validate_path_with_current_dir()` - Current directory handling
- `test_validate_path_multiple_subdirs()` - Deep directory structures
- `test_validate_path_with_dots_in_filename()` - Filenames with dots

## Running Tests

### Run all tests
```bash
cd backend
cargo test
```

### Run tests with verbose output
```bash
cargo test --verbose
```

### Run tests for a specific module
```bash
cargo test auth::tests
cargo test api::tests
cargo test image_handler::tests
cargo test tests  # for main.rs tests
```

### Run a specific test
```bash
cargo test test_verify_with_bcrypt_password
```

## Code Coverage

### Generate coverage locally (requires cargo-llvm-cov)

Install cargo-llvm-cov:
```bash
cargo install cargo-llvm-cov
```

Generate HTML coverage report:
```bash
cd backend
cargo llvm-cov --html
```

Generate LCOV format (for CI):
```bash
cargo llvm-cov --lcov --output-path lcov.info
```

### CI/CD Integration

The GitHub Actions workflow `.github/workflows/rust-tests.yml` automatically:
1. Runs all tests on every push and pull request
2. Generates code coverage reports
3. Uploads coverage to Codecov
4. Runs linting (clippy and rustfmt)

## Dependencies

Test-specific dependencies are defined in `Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3.15.0"
```

## Best Practices

1. **Test Isolation**: Each test creates its own temporary files using `tempfile` crate
2. **Async Tests**: Use `#[tokio::test]` for async test functions
3. **No Side Effects**: Tests don't modify system state or rely on external services
4. **Fast Execution**: All 39 tests complete in ~0.02 seconds
5. **Clear Naming**: Test names clearly describe what they test

## Future Improvements

Potential areas for additional test coverage:
- Integration tests for API endpoints with mock HTTP requests
- Performance benchmarks for thumbnail generation
- Load testing for concurrent authentication requests
- End-to-end tests with actual file system operations
