use crate::models::dfs::Dfs;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Result},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct UnlockRequest {
    path: PathBuf,
    exclusive: bool,
}

pub async fn unlock(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<UnlockRequest>,
) -> Result<impl IntoResponse> {
    // create file to the storage server
    let dfs = dfs.read().await;
    dfs.unlock(&payload.path, payload.exclusive)?;
    Ok(StatusCode::OK)
}
