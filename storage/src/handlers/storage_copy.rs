//! Copy a file from one storage server to local.

use crate::storage::Storage;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
    Json,
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

/// Represents the request payload for the storage copy operation.
#[derive(Debug, serde::Deserialize)]
pub struct StorageCopyRequest {
    path: PathBuf,
    server_ip: String,
    server_port: u16,
}

/// Represents the response payload for the storage copy operation.
#[derive(Debug, serde::Serialize)]
struct StorageCopyResponse {
    /// Indicates whether file copy was successful.
    success: bool,
}

/// Handles the storage copy.
pub async fn storage_copy(
    State(storage): State<Arc<Mutex<Storage>>>,
    axum::Json(payload): axum::Json<StorageCopyRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    let success = storage
        .copy(&payload.path, &payload.server_ip, payload.server_port)
        .await?;

    Ok(Json(StorageCopyResponse { success }))
}
