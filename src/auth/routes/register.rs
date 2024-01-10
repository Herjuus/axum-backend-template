use axum::http::StatusCode;
use axum::{Json};
use serde::{Deserialize, Serialize};
use crate::Tx;
use pwhash::bcrypt;
use crate::error::ApiError;

pub async fn register_user(mut tx: Tx, Json(payload): Json<Request>) -> Result<(StatusCode, String), ApiError> {
    let user = User {
        username: payload.username,
        email: payload.email,
        hashed_password: bcrypt::hash(payload.password).unwrap(),
    };

    let _result = sqlx::query(
        "INSERT INTO users (username, email, hashed_password) VALUES ($1, $2, $3)")
        .bind(user.username)
        .bind(user.email)
        .bind(user.hashed_password)
        .execute(&mut tx)
        .await.map_err(|_e| ApiError { status_code: StatusCode::NOT_ACCEPTABLE, message: "User with these credentials already exists.".to_string() })?;

    Ok((StatusCode::CREATED, "Successfully registered".to_string()))
}

#[derive(Deserialize)]
pub struct Request {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct User {
    username: String,
    email: String,
    hashed_password: String,
}