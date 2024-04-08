//! Write data to a file in the storage.
use crate::storage::Storage;
use axum::response::Result;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
#[derive(Deserialize, Debug)]
pub struct StorageWriteRequest {
    /// The path of the file to write to.
    pub path: PathBuf,
    /// The offset in the file to write to.
    pub offset: i64,
    /// The data to write.
    #[serde(with = "base64")]
    pub data: Vec<u8>,
}

/// Deserializes base64-encoded data.
mod base64 {
    use serde::Deserialize;
    use serde::Deserializer;
    /// Deserializes base64-encoded data.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::decode(base64.as_bytes()).map_err(|e| serde::de::Error::custom(e))
    }
}

/// Represents the response payload for writing data to a file in the storage.
#[derive(serde::Serialize)]
struct StorageWriteResponse {
    /// Indicates whether the write operation was successful.
    pub success: bool,
}

/// Handler function for writing data to a file in the storage.
pub async fn storage_write(
    State(storage): State<Arc<Mutex<Storage>>>,
    Json(payload): Json<StorageWriteRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    storage.write(&payload.path, payload.offset, payload.data)?;

    Ok(Json(StorageWriteResponse { success: true }))
}
