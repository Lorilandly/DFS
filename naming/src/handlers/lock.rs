use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct LockRequest {
    path: String,
}
#[derive(Debug, serde::Serialize)]
pub struct LockResponse {
    success: bool,
}

pub async fn lock(axum::Json(_payload): axum::Json<LockRequest>) -> impl IntoResponse {
    // create file to the storage server
    let response = LockResponse { success: true };

    (StatusCode::OK, axum::Json(response))
}
