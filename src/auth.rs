mod jwt;
pub mod routes;
pub mod middleware;

use serde::{Serialize};

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    hashed_password: String,
}
