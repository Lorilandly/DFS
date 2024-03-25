use axum::response::IntoResponse;

#[derive(Debug, serde::Deserialize)]
pub struct RegisterRequest {
    storage_ip: String,
    client_port: u16,
    command_port: u16,
    files: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct RegisterResponse {
    files: Vec<String>,
}

pub async fn register(axum::Json(payload): axum::Json<RegisterRequest>) -> impl IntoResponse {
    axum::Json(RegisterResponse {
        files: payload.files,
    })
}
