use axum::{Router, routing::get};

use server::{
    connection::{
        database_conn::{establish_read_connection, establish_write_connection},
        http_conn::create_http_client,
    },
    utils::logger::init_tracing,
};
use tracing::error;

async fn run_server() -> anyhow::Result<()> {
    let write_db = establish_write_connection().await;
    let read_db = establish_read_connection().await;

    let http_client = create_http_client().await.map_err(|e| {
        error!("Failed to create http client: {}", e);
        anyhow::anyhow!("Http client creation failed: {}", e)
    })?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_tracing();

    if let Err(err) = run_server().await {
        eprintln!("Application error: {}", err);
    }
}
