use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct IsValidPathRequest {
    path: String,
}

#[derive(Debug, serde::Serialize)]
pub struct IsValidPathResponse {
    success: bool,
}

pub async fn is_valid_path(
    axum::Json(_payload): axum::Json<IsValidPathRequest>,
) -> impl IntoResponse {
    let response = IsValidPathResponse { success: false };
    axum::Json(response)
}
