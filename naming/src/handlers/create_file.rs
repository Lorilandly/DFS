use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct CreateFileRequest {
    path: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateFileResponse {
    success: bool,
}

pub async fn create_file(axum::Json(_payload): axum::Json<CreateFileRequest>) -> impl IntoResponse {
    axum::Json(CreateFileResponse { success: true })
}
