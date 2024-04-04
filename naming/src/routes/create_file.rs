use crate::models::dfs::Dfs;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct CreateFileRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
struct CreateFileResponse {
    success: bool,
}

pub async fn create_file(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<CreateFileRequest>,
) -> Result<impl IntoResponse> {
    let mut dfs = dfs.write().await;
    let success = dfs.insert(&payload.path, false).await?;
    Ok(axum::Json(CreateFileResponse { success }))
}
