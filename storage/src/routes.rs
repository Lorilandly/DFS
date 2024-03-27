use crate::handlers;
use crate::storage::Storage;
use axum::routing::{post, Router};
use std::sync::{Arc, Mutex};

pub(super) fn client_routes(storage: Arc<Mutex<Storage>>) -> Router {
    Router::new()
        .route("/storage_size", post(handlers::storage_size::storage_size))
        .route("/storage_read", post(handlers::storage_read::storage_read))
        .route(
            "/storage_write",
            post(handlers::storage_write::storage_write),
        )
        .with_state(storage)
}

pub(super) fn command_routes(storage: Arc<Mutex<Storage>>) -> Router {
    Router::new()
        .route("/storage_create", post(|| async { "Hello, World!" }))
        .route("/storage_delete", post(|| async { "Hello, World!" }))
        .route("/storage_copy", post(|| async { "Hello, World!" }))
        .with_state(storage)
}
