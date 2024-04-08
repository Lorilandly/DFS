use crate::storage::Storage;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
#[derive(Debug, serde::Deserialize)]
pub struct StorageCreateRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
struct StorageCreateResponse {
    success: bool,
}

pub async fn storage_create(
    State(storage): State<Arc<Mutex<Storage>>>,
    axum::Json(payload): axum::Json<StorageCreateRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    let success = storage.create_file(&payload.path)?;
    Ok(axum::Json(StorageCreateResponse { success }))
}
