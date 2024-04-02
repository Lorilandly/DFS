use crate::models::dfs::Dfs;
use axum::{
    extract::State,
    response::{IntoResponse, Result},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
pub struct ListRequest {
    path: PathBuf,
}
#[derive(Debug, serde::Serialize)]
struct ListResponse {
    files: Vec<String>,
}

pub async fn list(
    State(dfs): State<Arc<RwLock<Dfs>>>,
    axum::Json(payload): axum::Json<ListRequest>,
) -> Result<impl IntoResponse> {
    // create file to the storage server
    let dfs = dfs.read().await;
    let files = dfs.list(&payload.path).await?;
    Ok(axum::Json(ListResponse { files }))
}
