mod register;
mod login;
mod validate;

use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, post};
// use crate::auth::get_users;
use crate::auth::middleware::jwt_middleware;
use crate::auth::routes::validate::validate_user;

pub fn auth_routes() -> Router {
    let routes = Router::new()
        // .route("/get_users", get(get_users))
        .route("/validate", get(validate_user))
        .route_layer(from_fn(jwt_middleware))
        .route("/register", post(register::register_user))
        .route("/login", post(login::login_user));
    (routes)
}