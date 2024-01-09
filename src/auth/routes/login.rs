use axum::http::StatusCode;
use axum::{debug_handler, Json};
use axum::body::HttpBody;
use serde::{Deserialize, Serialize};
use crate::error;
use crate::Tx;
use pwhash::bcrypt;
use crate::auth::jwt::generate_user_token;
use crate::error::ApiError;

pub async fn login_user(mut tx: Tx, Json(payload): Json<Request>) -> Result<(StatusCode, Json<Return>), ApiError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", payload.email)
        .fetch_one(&mut tx)
        .await.map_err(|e| ApiError { status_code: StatusCode::NOT_FOUND, message: "Incorrect email.".to_string() })?;

    let correct_password = bcrypt::verify(payload.password, user.hashed_password.as_str());

    if !correct_password {
        return Err(ApiError { status_code: StatusCode::NOT_ACCEPTABLE, message: "Incorrect password.".to_string() })
    }
 
    let token = generate_user_token(user.id).unwrap();

    let return_object = Return {
        Token: token,
        Message: "Logged in.".to_string(),
    };

    Ok((StatusCode::OK, Json(return_object)))
}

struct User {
    id: i32,
    username: String,
    email: String,
    hashed_password: String,
}

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct Return {
    Token: String,
    Message: String,
}
