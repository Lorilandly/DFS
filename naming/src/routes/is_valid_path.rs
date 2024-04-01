use crate::models::dfs::Dfs;
use axum::{extract::State, response::IntoResponse};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

#[derive(Debug, serde::Deserialize)]
pub struct IsValidPathRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
pub struct IsValidPathResponse {
    success: bool,
}

pub async fn is_valid_path(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<IsValidPathRequest>,
) -> impl IntoResponse {
    let dfs = dfs.write().unwrap();
    match dfs.is_dir(&payload.path) {
        Ok(res) => axum::Json(IsValidPathResponse { success: res }),
        Err(_) => axum::Json(IsValidPathResponse { success: false }),
    }
}
