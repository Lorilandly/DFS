use crate::models::dfs::Dfs;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct CreateDirRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
struct CreateDirResponse {
    success: bool,
}

pub async fn create_dir(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<CreateDirRequest>,
) -> Result<impl IntoResponse> {
    let mut dfs = dfs.write().await;
    let success = dfs.insert(&payload.path, true).await?;
    Ok(axum::Json(CreateDirResponse { success }))
}
