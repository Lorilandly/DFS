use std::sync::{Arc, Mutex};

use axum::{extract::State, response::IntoResponse};

use crate::dfs::{Dfs, Storage};

#[derive(Debug, serde::Deserialize)]
pub struct RegisterRequest {
    storage_ip: String,
    client_port: u16,
    command_port: u16,
    files: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct RegisterResponse {
    files: Vec<String>,
}

pub async fn register(
    State(dfs): State<Arc<Mutex<Dfs>>>,
    axum::Json(payload): axum::Json<RegisterRequest>,
) -> impl IntoResponse {
    let mut dfs = dfs.lock().unwrap();
    if dfs.storage.insert(Storage {
        storage_ip: payload.storage_ip.clone(),
        client_port: payload.client_port,
        command_port: payload.command_port,
    }) {
        tracing::info!("Registered storage: {:?}", payload);
        axum::Json(RegisterResponse {
            files: dfs.fs.insert_files(payload.files),
        })
        .into_response()
    } else {
        tracing::warn!("Storage already registered: {:?}", payload);
        (
            axum::http::StatusCode::CONFLICT,
            axum::Json(super::exception_return::ExceptionReturn::new(
                "IllegalStateException",
                "This storage server is already registered.",
            )),
        )
            .into_response()
    }
}
