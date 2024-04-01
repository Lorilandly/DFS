use crate::models::dfs::Dfs;
use axum::{extract::State, response::IntoResponse};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

#[derive(Debug, serde::Deserialize)]
pub struct ListRequest {
    path: PathBuf,
}
#[derive(Debug, serde::Serialize)]
pub struct ListResponse {
    files: Vec<String>,
    success: bool,
}

pub async fn list(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<ListRequest>,
) -> impl IntoResponse {
    // create file to the storage server
    let dfs = dfs.read().unwrap();
    match dfs.list(&payload.path) {
        Ok(files) => axum::Json(ListResponse {
            files,
            success: true,
        })
        .into_response(),
        Err(e) => e.into_response(),
    }
}
