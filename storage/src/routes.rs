use axum::routing::{post, Router};

pub(super) fn client_routes() -> Router {
    Router::new()
        .route("/storage_size", post(|| async { "Hello, World!" }))
        .route("/storage_read", post(|| async { "Hello, World!" }))
        .route("/storage_write", post(|| async { "Hello, World!" }))
}

pub(super) fn command_routes() -> Router {
    Router::new()
        .route("/storage_create", post(|| async { "Hello, World!" }))
        .route("/storage_delete", post(|| async { "Hello, World!" }))
        .route("/storage_copy", post(|| async { "Hello, World!" }))
}
