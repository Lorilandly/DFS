//! Routes for the storage service.
use crate::handlers;
use crate::storage::Storage;
use axum::routing::{post, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

/// a router with the storage service routes.
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

///  a router with the storage service routes.
pub(super) fn command_routes(storage: Arc<Mutex<Storage>>) -> Router {
    Router::new()
        .route(
            "/storage_create",
            post(handlers::storage_create::storage_create),
        )
        .route(
            "/storage_delete",
            post(handlers::storage_delete::storage_delete),
        )
        .route("/storage_copy", post(handlers::storage_copy::storage_copy))
        .with_state(storage)
}
