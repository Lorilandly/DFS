use crate::dfs::Dfs;
use axum::{extract::State, response::IntoResponse};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug, serde::Deserialize)]
pub struct IsDirRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
pub struct IsDirResponse {
    success: bool,
}

pub async fn is_directory(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<IsDirRequest>,
) -> impl IntoResponse {
    let dfs = dfs.read().unwrap();
    match dfs.fs.is_dir(&payload.path) {
        Ok(res) => axum::Json(IsDirResponse { success: res }).into_response(),
        Err(e) => e.into_response(),
    }
}
