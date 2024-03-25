use crate::handlers;
use axum::routing::{post, Router};

pub(super) fn service_routes() -> Router {
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
}

pub(super) fn registration_routes() -> Router {
    Router::new().route("/register", post(handlers::register::register))
}
