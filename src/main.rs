mod config;
mod db;
mod app;
mod repository;
mod models;
mod routes;

use crate::app::create_app;
use crate::db::connect_db;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = connect_db(&database_url).await.expect("Failed to connect DB");

    let app = create_app(pool);
    let addr = "0.0.0.0:8000".parse().unwrap();
    println!("✅ Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}