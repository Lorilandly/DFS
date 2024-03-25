use crate::dfs::{Dfs, Storage};
use crate::handlers;
use axum::routing::{post, Router};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub(super) fn service_routes(dfs: Arc<Mutex<Dfs>>) -> Router {
    Router::new()
        .route(
            "/is_valid_path",
            post(handlers::is_valid_path::is_valid_path),
        )
        .route("/get_storage", post(handlers::get_storage::get_storage))
        .route("/delete", post(handlers::delete::delete))
        .route(
            "/create_directory",
            post(handlers::create_directory::create_dir),
        )
        .route("/create_file", post(handlers::create_file::create_file))
        .route("/list", post(handlers::list::list))
        .route("/is_directory", post(handlers::is_directory::is_directory))
        .route("/unlock", post(handlers::unlock::unlock))
        .route("/lock", post(handlers::lock::lock))
        .with_state(dfs)
}

pub(super) fn registration_routes(dfs: Arc<Mutex<Dfs>>) -> Router {
    Router::new()
        .route("/register", post(handlers::register::register))
        .with_state(dfs)
}
