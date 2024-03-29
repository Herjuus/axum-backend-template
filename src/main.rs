mod auth;
pub mod error;

use axum::{Router};
use dotenv::dotenv;
use std::error::Error;
use sqlx::PgPool;
use crate::auth::routes::auth_routes;

pub type Tx = axum_sqlx_tx::Tx<sqlx::Postgres>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let pool = PgPool::connect(std::env::var("DATABASE_URL").unwrap().as_str()).await?;

    let app = Router::new()
        .nest("/auth", auth_routes())
        .layer(axum_sqlx_tx::Layer::new(pool.clone()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
