use crate::models::dfs::Dfs;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Result},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct LockRequest {
    path: PathBuf,
    exclusive: bool,
}

pub async fn lock(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<LockRequest>,
) -> Result<impl IntoResponse> {
    // create file to the storage server
    let dfs = dfs.read().await;
    dfs.lock(&payload.path, payload.exclusive).await?;
    Ok(StatusCode::OK)
}
