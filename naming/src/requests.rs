use crate::{exception_return::ExceptionReturn, models::storage::Storage};
use axum::{http::StatusCode, Json};
use reqwest;
use std::path::Path;

type Result<T> = std::result::Result<T, (StatusCode, Json<ExceptionReturn>)>;

pub async fn storage_create(storage: &Storage, path: &Path) -> Result<bool> {
    let client = reqwest::Client::new();
    let _res = client
        .post(format!(
            "http://{}:{}/storage_create",
            storage.storage_ip, storage.command_port
        ))
        .body(format!("{{\"path\": \"{}\"}}", path.to_str().unwrap()))
        .send()
        .await
        .unwrap();
    Ok(true) // todo: use the response from the storage server
}

pub async fn storage_delete(storage: &Storage, path: &Path) -> Result<bool> {
    let client = reqwest::Client::new();
    let _res = client
        .post(format!(
            "http://{}:{}/storage_delete",
            storage.storage_ip, storage.command_port
        ))
        .body(format!("{{\"path\": \"{}\"}}", path.to_str().unwrap()))
        .send()
        .await
        .unwrap();
    Ok(true) // todo: use the response from the storage server
}

pub async fn storage_copy(target: &Storage, source: &Storage, path: &Path) -> Result<bool> {
    let client = reqwest::Client::new();
    let _res = client
        .post(format!(
            "http://{}:{}/storage_copy",
            target.storage_ip, target.command_port
        ))
        .body(format!(
            "{{\"path\": \"{}\", \"server_ip\": \"{}\", \"server_port\": {}}}",
            path.display(),
            source.storage_ip,
            source.client_port,
        ))
        .send()
        .await
        .unwrap();
    Ok(true) // todo: use the response from the storage server
}
