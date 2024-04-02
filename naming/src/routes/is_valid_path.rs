use crate::models::dfs::Dfs;
use axum::response::IntoResponse;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct IsValidPathRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
struct IsValidPathResponse {
    success: bool,
}

pub async fn is_valid_path(
    axum::Json(payload): axum::Json<IsValidPathRequest>,
) -> impl IntoResponse {
    axum::Json(IsValidPathResponse {
        success: Dfs::is_valid_path(&payload.path),
    })
}
