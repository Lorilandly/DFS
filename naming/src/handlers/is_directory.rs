use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct IsDirRequest {
    path: String,
}

#[derive(Debug, serde::Serialize)]
pub struct IsDirResponse {
    success: bool,
}

pub async fn is_directory(axum::Json(_payload): axum::Json<IsDirRequest>) -> impl IntoResponse {
    let response = IsDirResponse { success: true };
    axum::Json(response)
}
