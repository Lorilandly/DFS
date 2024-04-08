//! This module contains functions that send requests to the storage service.
use crate::handlers::{
    exception_return::ExceptionReturn,
    storage_read::{StorageReadRequest, StorageReadResponse},
    storage_size::{StorageSizeRequest, StorageSizeResponse},
};

use reqwest::{self, Error};
use std::path::PathBuf;

/// Sends a request to the storage service to get the size of a file.
pub async fn get_file_size(
    path: PathBuf,
    ip: String,
    port: i16,
) -> Result<StorageSizeResponse, ExceptionReturn> {
    let client = reqwest::Client::new();
    let res = client
        .post(&format!("http://{}:{}/storage_size", ip, port))
        .json(&StorageSizeRequest { path })
        .send()
        .await
        .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.json::<StorageSizeResponse>().await.unwrap()),
        _ => Err(res.json::<ExceptionReturn>().await.unwrap()),
    }
}

/// Sends a request to the storage service to read a file.
pub async fn read_file(
    payload: StorageReadRequest,
    ip: String,
    port: i16,
) -> Result<StorageReadResponse, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(&format!("http://{}:{}/storage_read", ip, port))
        .json(&payload)
        .send()
        .await
        .unwrap();

    res.json::<StorageReadResponse>().await
}
