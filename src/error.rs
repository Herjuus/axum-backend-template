use axum::http::{header, StatusCode};
use axum::Json;
use axum::response::{IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (status_code,[(header::CONTENT_TYPE,"application/json")],Json(json!({ "statuscode": self.status_code.as_u16(), "message": self.message }))).into_response()
    }
}