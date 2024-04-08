//! Create a file in the storage.

use crate::storage::Storage;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

/// Represents the request payload for creating a file in the storage.
#[derive(Debug, serde::Deserialize)]
pub struct StorageCreateRequest {
    path: PathBuf,
}

/// Represents the response payload for creating a file in the storage.
#[derive(Debug, serde::Serialize)]
struct StorageCreateResponse {
    success: bool,
}

/// Handler function for creating a file in the storage.
pub async fn storage_create(
    State(storage): State<Arc<Mutex<Storage>>>,
    axum::Json(payload): axum::Json<StorageCreateRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    let success = storage.create_file(&payload.path)?;
    Ok(axum::Json(StorageCreateResponse { success }))
}
