use axum::{Router, routing::get};
use std::net::SocketAddr;

use axum_app_config::ServerConfig;
use server::{
    api::router::api_routes, appstate::AppState, connection::{
        database_conn::{establish_read_connection, establish_write_connection},
        http_conn::create_http_client,
    }, utils::logger::init_tracing
};
use tracing::error;

async fn run_server() -> anyhow::Result<()> {
    let write_db = establish_write_connection().await;
    let read_db = establish_read_connection().await;

    let http_client = create_http_client().await.map_err(|e| {
        error!("Failed to create http client: {}", e);
        anyhow::anyhow!("Http client creation failed: {}", e)
    })?;

    let server_url = format!(
        "{}:{}",
        &ServerConfig::get().server_host,
        &ServerConfig::get().server_port
    );

    let state = AppState {
        http_client,
        write_db,
        read_db,
    };

    let app = Router::new()
        .merge(api_routes(state.clone()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

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
