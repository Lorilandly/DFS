mod create_directory;
mod create_file;
mod delete;
mod get_storage;
mod is_directory;
mod is_valid_path;
mod list;
mod lock;
mod register;
mod unlock;

use crate::models::dfs::Dfs;
use axum::routing::{post, Router};
use std::sync::Arc;
use tokio::sync::RwLock;

pub(super) fn service_routes(dfs: Arc<RwLock<Dfs>>) -> Router {
    Router::new()
        .route("/is_valid_path", post(is_valid_path::is_valid_path))
        .route("/get_storage", post(get_storage::get_storage))
        .route("/delete", post(delete::delete))
        .route("/create_directory", post(create_directory::create_dir))
        .route("/create_file", post(create_file::create_file))
        .route("/list", post(list::list))
        .route("/is_directory", post(is_directory::is_directory))
        .route("/unlock", post(unlock::unlock))
        .route("/lock", post(lock::lock))
        .with_state(dfs)
}

pub(super) fn registration_routes(dfs: Arc<RwLock<Dfs>>) -> Router {
    Router::new()
        .route("/register", post(register::register))
        .with_state(dfs)
}
