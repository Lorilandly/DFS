use crate::models::dfs::Dfs;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct IsDirRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
struct IsDirResponse {
    success: bool,
}

pub async fn is_directory(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<IsDirRequest>,
) -> Result<impl IntoResponse> {
    let dfs = dfs.read().await;
    let success = dfs.is_dir(&payload.path)?;
    Ok(axum::Json(IsDirResponse { success }))
}
