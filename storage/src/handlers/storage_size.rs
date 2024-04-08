use crate::storage::Storage;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
    Json,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Deserialize, Debug, Serialize)]
pub struct StorageSizeRequest {
    pub path: PathBuf,
}

#[derive(serde::Serialize, Debug, Deserialize)]
pub struct StorageSizeResponse {
    pub size: u64,
}

pub async fn storage_size(
    State(storage): State<Arc<Mutex<Storage>>>,
    Json(payload): Json<StorageSizeRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    let size = storage.get_file_size(&payload.path)?;

    Ok(Json(StorageSizeResponse { size }))
}
