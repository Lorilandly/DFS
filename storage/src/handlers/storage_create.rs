use crate::storage::Storage;
use axum::{extract::State, response::IntoResponse, Json};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
#[derive(Debug, serde::Deserialize)]
pub struct StorageCreateRequest {
    path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
pub struct StorageCreateResponse {
    success: bool,
}

pub async fn storage_create(
    State(storage): State<Arc<Mutex<Storage>>>,
    axum::Json(payload): axum::Json<StorageCreateRequest>,
) -> impl axum::response::IntoResponse {
    let storage = storage.lock().unwrap();
    match storage.create_file(&payload.path) {
        Ok(res) => axum::Json(StorageCreateResponse { success: res }).into_response(),
        Err(e) => e.into_response(),
    }
}
