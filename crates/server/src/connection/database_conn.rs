use std::time::Duration;

use axum_app_config::ServerConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

struct DbConfig<'a> {
    user: &'a str,
    password: &'a str,
    host: &'a str,
    port: &'a str,
    name: &'a str,
    max_connections: u32,
    min_connections: u32,
}

async fn establish_connection_with_config(
    db_config: DbConfig<'_>,
    label: &str,
) -> DatabaseConnection {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.name,
    );

    let _masked_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user,
        "*".repeat(db_config.password.len()),
        db_config.host,
        db_config.port,
        db_config.name
    );

    info!(
        "Establishing {} database connection to {}",
        label, _masked_url
    );

    let mut options = ConnectOptions::new(database_url);
    options
        .min_connections(db_config.min_connections)
        .max_connections(db_config.max_connections)
        .connect_timeout(std::time::Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(std::time::Duration::from_secs(300))
        .sqlx_logging(false);

    match Database::connect(options).await {
        Ok(conn) => {
            info!("Successfully established {} database connection", label);
            conn
        }
        Err(err) => {
            panic!("Failed to establish {} database connection: {}", label, err);
        }
    }
}

pub async fn establish_write_connection() -> DatabaseConnection {
    let db_config = ServerConfig::get();
    establish_connection_with_config(
        DbConfig {
            user: &db_config.db_write_user,
            password: &db_config.db_write_password,
            host: &db_config.db_write_host,
            port: &db_config.db_write_port,
            name: &db_config.db_write_name,
            max_connections: db_config.db_write_max_connection,
            min_connections: db_config.db_write_min_connection,
        },
        "write",
    )
    .await
}

pub async fn establish_read_connection() -> DatabaseConnection {
    let db_config = ServerConfig::get();
    establish_connection_with_config(
        DbConfig {
            user: &db_config.db_read_user,
            password: &db_config.db_read_password,
            host: &db_config.db_read_host,
            port: &db_config.db_read_port,
            name: &db_config.db_read_name,
            max_connections: db_config.db_read_max_connection,
            min_connections: db_config.db_read_min_connection,
        },
        "read",
    )
    .await
}
