use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct ListRequest {
    path: String,
}
#[derive(Debug, serde::Serialize)]
pub struct ListResponse {
    files: Vec<String>,
    success: bool,
}

pub async fn list(axum::Json(_payload): axum::Json<ListRequest>) -> impl IntoResponse {
    // create file to the storage server
    let response = ListResponse {
        files: vec![],
        success: true,
    };

    axum::Json(response)
}
