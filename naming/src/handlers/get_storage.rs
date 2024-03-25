use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct GetStorageRequest {
    ip: String,
}

#[derive(Debug, serde::Serialize)]
pub struct GetStorageResponse {
    message: String,
    success: bool,
}

pub async fn get_storage(axum::Json(_payload): axum::Json<GetStorageRequest>) -> impl IntoResponse {
    // TODO: Delete the file
    let response = GetStorageResponse {
        message: "Storage File".to_string(),
        success: true,
    };

    axum::Json(response)
}
