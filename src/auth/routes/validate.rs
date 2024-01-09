use axum::http::StatusCode;
use axum::Json;
use crate::error::ApiError;
use crate::Tx;

pub async fn validate_user(mut tx: Tx) -> Result<(StatusCode, String), ApiError> {
    Ok((StatusCode::OK, "Success.".to_string()))
}
