mod jwt;
pub mod routes;
pub mod middleware;

use std::error::Error;
use axum::{http::StatusCode, Json};
use axum::body::HttpBody;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::{error, Tx};
use crate::error::ApiError;

// pub async fn get_users(mut tx: Tx) -> Result<(StatusCode, Json<Vec<User>>), ApiError> {
//     let users = sqlx::query_as!(User, "SELECT * FROM users")
//         .fetch_all(&mut tx)
//         .await
//         .map_err(|e| ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Connection to database failed.".to_string() })?;
//
//     Ok((StatusCode::OK, Json(users)))
// }
//
#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    hashed_password: String,
}
