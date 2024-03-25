use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct UnlockRequest {
    path: String,
}

#[derive(Debug, serde::Serialize)]
pub struct UnLockResponse {
    success: bool,
}

pub async fn unlock(axum::Json(_payload): axum::Json<UnlockRequest>) -> impl IntoResponse {
    // create file to the storage server
    let response = UnLockResponse { success: true };

    (StatusCode::OK, axum::Json(response))
}
