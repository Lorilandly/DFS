use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct CreateDirRequest {
    path: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateDirResponse {
    success: bool,
}

pub async fn create_dir(axum::Json(_payload): axum::Json<CreateDirRequest>) -> impl IntoResponse {
    let response = CreateDirResponse { success: true };
    (StatusCode::OK, axum::Json(response))
}
