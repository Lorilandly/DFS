use crate::storage::Storage;
use axum::{extract::State, response::IntoResponse};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, serde::Deserialize)]
pub struct StorageDeleteRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
pub struct StorageDeleteResponse {
    success: bool,
}

pub async fn storage_delete(
    State(storage): State<Arc<Mutex<Storage>>>,
    axum::Json(payload): axum::Json<StorageDeleteRequest>,
) -> impl IntoResponse {
    let storage = storage.lock().unwrap();
    match storage.delete_file(&payload.path) {
        Ok(res) => axum::Json(StorageDeleteResponse { success: res }).into_response(),
        Err(e) => e.into_response(),
    }
}
