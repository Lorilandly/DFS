use axum::response::IntoResponse;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct LockRequest {
    path: PathBuf,
    exclusive: bool,
}

pub async fn lock(axum::Json(_payload): axum::Json<LockRequest>) -> impl IntoResponse {
    // create file to the storage server
    axum::http::StatusCode::OK
}
