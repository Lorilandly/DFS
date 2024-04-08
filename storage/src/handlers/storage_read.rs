//! Read data from a file in the storage
use crate::handlers::exception_return::ExceptionReturn;
use crate::storage::Storage;
use axum::{extract::State, response::IntoResponse, Json};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

/// Represents the request payload for reading data from a file in the storage.
#[derive(Deserialize, Debug, Serialize)]
pub struct StorageReadRequest {
    /// The path of the file to read.
    pub path: PathBuf,
    /// The offset from which to start reading.
    pub offset: i64,
    /// The length of data to read.
    pub length: i64,
}

/// Represents the response payload for reading data from a file in the storage.
#[derive(Serialize, Debug, Deserialize)]
pub struct StorageReadResponse {
    /// The data read from the file.
    pub data: String,
}

/// Handler function for reading data from a file in the storage.
pub async fn storage_read(
    State(storage): State<Arc<Mutex<Storage>>>,
    Json(payload): Json<StorageReadRequest>,
) -> impl IntoResponse {
    if payload.offset < 0 || payload.length < 0 {
        return Json(ExceptionReturn::new(
            "IndexOutOfBoundsException",
            "Offset or length cannot be negative",
        ))
        .into_response();
    }
    let storage = storage.lock().await;
    match storage.read(&payload.path, payload.offset as u64, payload.length as u64) {
        Ok(data) => Json(StorageReadResponse {
            data: BASE64_STANDARD.encode(data),
        })
        .into_response(),
        Err(e) => e.into_response(),
    }
}
