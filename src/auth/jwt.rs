use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, Header, EncodingKey, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::auth::User;
use crate::error::ApiError;

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    user_id: i32,
}

pub fn generate_user_token(uid: i32) -> Result<String, ApiError> {
    let mut time = Utc::now();
    let expires_in = Duration::minutes(30);
    time += expires_in;

    let claims = Claims {
        exp: time.timestamp() as usize,
        user_id: uid,
    };

    let key = EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref());

    encode(&Header::default(), &claims, &key).map_err(|_e| ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Could not create a user token.".to_string() })
}

pub async fn validate_user_token(token: &str) -> Result<User, ApiError> {
    let pool = PgPool::connect(std::env::var("DATABASE_URL")
        .unwrap().as_str())
        .await.map_err(|_e| ApiError { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: "Could not connect to the database.".to_string() })?;

    let decoded_token = decode::<Claims>(&token, &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()), &Validation::new(Algorithm::HS256))
        .map_err(|e| ApiError { status_code: StatusCode::UNAUTHORIZED, message: match e.to_string().as_str() {
            "ExpiredSignature" => {"Token expired.".to_string()},
            _ => {"Invalid token.".to_string()}
        }})?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", decoded_token.claims.user_id)
        .fetch_one(&pool.clone())
        .await
        .map_err(|_e| ApiError { status_code: StatusCode::UNAUTHORIZED, message: "No user found with this token.".to_string() })?;

    Ok(user)
}