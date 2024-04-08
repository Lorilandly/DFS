//! Get the size of a file in the storage.
use crate::storage::Storage;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
    Json,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

/// Represents the request payload for getting the size of a file in the storage.
#[derive(Deserialize, Debug, Serialize)]
pub struct StorageSizeRequest {
    /// The path of the file to get the size of.
    pub path: PathBuf,
}

/// Represents the response payload for getting the size of a file in the storage.
#[derive(serde::Serialize, Debug, Deserialize)]
pub struct StorageSizeResponse {
    /// The size of the file in bytes.
    pub size: u64,
}

/// Handler function for getting the size of a file in the storage.
pub async fn storage_size(
    State(storage): State<Arc<Mutex<Storage>>>,
    Json(payload): Json<StorageSizeRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    let size = storage.get_file_size(&payload.path)?;
    Ok(Json(StorageSizeResponse { size }))
}
