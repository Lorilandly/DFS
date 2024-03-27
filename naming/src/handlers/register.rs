use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use axum::{extract::State, response::IntoResponse};

use crate::dfs::{Dfs, Storage};

#[derive(Debug, serde::Deserialize)]
pub struct RegisterRequest {
    storage_ip: String,
    client_port: u16,
    command_port: u16,
    files: Vec<PathBuf>,
}

#[derive(Debug, serde::Serialize)]
pub struct RegisterResponse {
    files: Vec<PathBuf>,
}

pub async fn register(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<RegisterRequest>,
) -> impl IntoResponse {
    let mut dfs = dfs.write().unwrap();
    let storage = Arc::new(Storage {
        storage_ip: payload.storage_ip.clone(),
        client_port: payload.client_port,
        command_port: payload.command_port,
    });
    if dfs.storage.insert(storage.clone()) {
        tracing::info!("Registered storage: {:?}", payload);
        axum::Json(RegisterResponse {
            files: dfs.insert_files(payload.files, storage.clone()),
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
