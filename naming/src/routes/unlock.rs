use axum::response::IntoResponse;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct UnlockRequest {
    path: PathBuf,
    exclusive: bool,
}

pub async fn unlock(axum::Json(_payload): axum::Json<UnlockRequest>) -> impl IntoResponse {
    // create file to the storage server
    axum::http::StatusCode::OK
}
