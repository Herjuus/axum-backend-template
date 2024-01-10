
use axum::http::{Request, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use crate::auth::jwt::validate_user_token;
use crate::error::ApiError;

pub async fn jwt_middleware<T>(mut req: Request<T>, next: Next<T>) -> Result<Response, ApiError> {
    let token = req.headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok() )
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let token = token.ok_or_else(|| {
        return Err(ApiError { status_code: StatusCode::UNAUTHORIZED, message: "No token provided.".to_string() })
    }).map_err(|_e: Result<T, ApiError>| ApiError { status_code: StatusCode::UNAUTHORIZED, message: "No token provided.".to_string() })?;

    let user = validate_user_token(token.as_str()).await?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}