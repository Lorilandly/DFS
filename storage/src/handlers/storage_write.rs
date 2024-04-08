use crate::{handlers::exception_return::ExceptionReturn, storage::Storage};
use axum::response::Result;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
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
struct StorageWriteResponse {
    pub success: bool,
}

pub async fn storage_write(
    State(storage): State<Arc<Mutex<Storage>>>,
    Json(payload): Json<StorageWriteRequest>,
) -> Result<impl IntoResponse> {
    let storage = storage.lock().await;
    storage.write(&payload.path, payload.offset, payload.data)?;

    Ok(Json(StorageWriteResponse { success: true }))
}
