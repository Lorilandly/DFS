use crate::handlers::exception_return::ExceptionReturn;
use crate::storage::Storage;
use axum::{extract::State, response::IntoResponse, Json};
use base64::prelude::*;
use serde::Deserialize;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Deserialize, Debug)]
pub struct StorageReadRequest {
    pub path: PathBuf,
    pub offset: i64,
    pub length: i64,
}

#[derive(serde::Serialize, Debug)]
pub struct StorageReadResponse {
    pub data: String,
}

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
    let storage = storage.lock().unwrap();
    match storage.read(&payload.path, payload.offset as u64, payload.length as u64) {
        Ok(data) => Json(StorageReadResponse {
            data: BASE64_STANDARD.encode(data),
        })
        .into_response(),
        Err(e) => e.into_response(),
    }
}
