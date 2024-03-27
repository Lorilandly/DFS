use crate::dfs::Dfs;
use axum::{extract::State, response::IntoResponse};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct CreateDirRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateDirResponse {
    success: bool,
}

pub async fn create_dir(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<CreateDirRequest>,
) -> impl IntoResponse {
    let mut dfs = dfs.write().unwrap();
    match dfs.insert(&payload.path, true) {
        Ok(res) => axum::Json(CreateDirResponse { success: res }).into_response(),
        Err(e) => e.into_response(),
    }
}
