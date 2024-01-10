use axum::http::StatusCode;
use crate::error::ApiError;

pub async fn validate_user() -> Result<(StatusCode, String), ApiError> {
    Ok((StatusCode::OK, "Success.".to_string()))
}
