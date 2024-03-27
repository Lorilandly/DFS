use crate::dfs::Dfs;
use axum::{extract::State, response::IntoResponse};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct CreateFileRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateFileResponse {
    success: bool,
}

pub async fn create_file(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<CreateFileRequest>,
) -> impl IntoResponse {
    let mut dfs = dfs.write().unwrap();

    match dfs.fs.insert(&payload.path, false) {
        Ok(res) => axum::Json(CreateFileResponse { success: res }).into_response(),
        Err(e) => e.into_response(),
    }
}
