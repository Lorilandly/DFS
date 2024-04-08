use crate::storage::Storage;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize)]
pub struct StorageDeleteRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
struct StorageDeleteResponse {
    success: bool,
}

pub async fn storage_delete(
    State(storage): State<Arc<Mutex<Storage>>>,
    axum::Json(payload): axum::Json<StorageDeleteRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    let success = storage.delete_file(&payload.path)?;
    Ok(axum::Json(StorageDeleteResponse { success }))
}
