//! Delete a file from the storage.
use crate::storage::Storage;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

/// Represents the request payload for deleting a file from the storage.
#[derive(Debug, serde::Deserialize)]
pub struct StorageDeleteRequest {
    /// The path of the file to delete.
    path: PathBuf,
}

/// Represents the response payload for deleting a file from the storage.
#[derive(Debug, serde::Serialize)]
struct StorageDeleteResponse {
    /// Indicates whether file deletion was successful.
    success: bool,
}

/// Handler function for deleting a file from the storage.
pub async fn storage_delete(
    State(storage): State<Arc<Mutex<Storage>>>,
    axum::Json(payload): axum::Json<StorageDeleteRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    let success = storage.delete_file(&payload.path)?;
    Ok(axum::Json(StorageDeleteResponse { success }))
}
