use crate::models::dfs::Dfs;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
    Json,
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct DeleteRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
pub struct DeleteResponse {
    success: bool,
}

pub async fn delete(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<DeleteRequest>,
) -> Result<impl IntoResponse> {
    // Delete the file
    let mut dfs = dfs.write().await;
    let success = dfs.delete(&payload.path).await?;
    Ok(Json(DeleteResponse { success }))
}
