use crate::storage::Storage;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Deserialize, Debug)]
pub struct StorageSizeRequest {
    pub path: PathBuf,
}

#[derive(serde::Serialize, Debug)]
pub struct StorageSizeResponse {
    pub size: u64,
}

pub async fn storage_size(
    State(storage): State<Arc<Mutex<Storage>>>,
    Json(payload): Json<StorageSizeRequest>,
) -> impl IntoResponse {
    let storage = storage.lock().unwrap();
    match storage.get_file_size(&payload.path) {
        Ok(size) => Json(StorageSizeResponse { size }).into_response(),
        Err(e) => e.into_response(),
    }
}
