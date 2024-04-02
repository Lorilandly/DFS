use crate::models::dfs::Dfs;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct GetStorageRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
struct GetStorageResponse {
    server_ip: String,
    server_port: u16,
}

pub async fn get_storage(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<GetStorageRequest>,
) -> Result<impl IntoResponse> {
    let dfs = dfs.read().await;
    let storage = dfs.get_storage(&payload.path)?;
    Ok(axum::Json(GetStorageResponse {
        server_ip: storage.storage_ip.clone(),
        server_port: storage.client_port,
    }))
}
