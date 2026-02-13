use axum::{Router, routing::get};

use server::{connection::database_conn::{establish_read_connection, establish_write_connection}, utils::logger::init_tracing};

async fn run_server() -> anyhow::Result<()> {
    let write_db = establish_write_connection().await;
    let read_db = establish_read_connection().await;
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
