use crate::{handlers::exception_return::ExceptionReturn, storage::Storage};
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Deserialize, Debug)]
pub struct StorageWriteRequest {
    pub path: PathBuf,
    pub offset: i64,
    #[serde(with = "base64")]
    pub data: Vec<u8>,
}

mod base64 {
    use serde::Deserialize;
    use serde::Deserializer;
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::decode(base64.as_bytes()).map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(serde::Serialize)]
pub struct StorageWriteResponse {
    pub success: bool,
}

pub async fn storage_write(
    State(storage): State<Arc<Mutex<Storage>>>,
    Json(payload): Json<StorageWriteRequest>,
) -> impl IntoResponse {
    if payload.offset < 0 {
        return Json(ExceptionReturn::new(
            "IndexOutOfBoundsException",
            "Offset or length cannot be negative",
        ))
        .into_response();
    }
    let storage = storage.lock().unwrap();
    match storage.write(&payload.path, payload.offset as u64, payload.data) {
        Ok(_) => Json(StorageWriteResponse { success: true }).into_response(),
        Err(e) => e.into_response(),
    }
}
