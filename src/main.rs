mod config;
mod db;
mod app;
mod repository;
mod models;
mod routes;
mod mqtt;


use crate::app::create_app;
use crate::db::connect_db;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::join;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = connect_db(&database_url).await.expect("Failed to connect DB");

    let shared_state = Arc::new(db::AppState { db: Arc::new(pool.clone()) });

    // Create app server
    let app = create_app(pool);

    let addr = "0.0.0.0:8000".parse().unwrap();
    println!("✅ Server running on {}", addr);

    // 🔥 Start both servers concurrently
    let http_server = axum::Server::bind(&addr)
        .serve(app.into_make_service());

    let mqtt_client = mqtt::mqtt_client::start_mqtt_client(shared_state.clone());

    // Run both concurrently
    join!(
        http_server,
        mqtt_client
    );
}

