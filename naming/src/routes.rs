use axum::routing::{post, Router};

pub(super) fn service_routes() -> Router {
    Router::new()
        .route("/is_valid_path", post(|| async { "Hello, World!" }))
        .route("/get_storage", post(|| async { "Hello, World!" }))
        .route("/delete", post(|| async { "Hello, World!" }))
        .route("/create_directory", post(|| async { "Hello, World!" }))
        .route("/create_file", post(|| async { "Hello, World!" }))
        .route("/list", post(|| async { "Hello, World!" }))
        .route("/is_directory", post(|| async { "Hello, World!" }))
        .route("/unlock", post(|| async { "Hello, World!" }))
        .route("/lock", post(|| async { "Hello, World!" }))
}

pub(super) fn registration_routes() -> Router {
    Router::new().route("/register", post(|| async { "Hello, World!" }))
}
