use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct DeleteRequest {
    path: String,
}

#[derive(Debug, serde::Serialize)]
pub struct DeleteResponse {
    status: String,
}

pub async fn delete(axum::Json(payload): axum::Json<DeleteRequest>) -> impl IntoResponse {
    let path = payload.path;

    // TODO: Delete the file

    let status = format!("Deleted {}", path);
    let resposne = DeleteResponse { status };
    (StatusCode::OK, axum::Json(resposne))
}
