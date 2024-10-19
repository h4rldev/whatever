use backend::{db::migrate::do_migration, register_tracing};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    register_tracing();
    if dotenv().is_err() {
        info!("No .env file found, using defaults");
    }
    do_migration().await;
    let app = axum::Router::new().route("/", axum::routing::get(index));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn index() -> String {
    "Hello, world!".to_string()
}
