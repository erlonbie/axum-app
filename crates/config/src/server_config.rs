use dotenvy::dotenv;
use std::env;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub is_dev: bool,

    // Write DB (Primary)
    pub db_write_host: String,
    pub db_write_port: String,
    pub db_write_name: String,
    pub db_write_user: String,
    pub db_write_password: String,
    pub db_write_max_connection: u32,
    pub db_write_min_connection: u32,

    // Read DB (Replica)
    pub db_read_host: String,
    pub db_read_port: String,
    pub db_read_name: String,
    pub db_read_user: String,
    pub db_read_password: String,
    pub db_read_max_connection: u32,
    pub db_read_min_connection: u32,

    pub server_host: String,
    pub server_port: String,
}

// Automatically initialized with LazyLock
static CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| {
    dotenv().ok();

    let mut errors: Vec<String> = Vec::new();

    macro_rules! require {
        ($name:expr) => {
            env::var($name).unwrap_or_else(|_| {
                errors.push(format!("  - {} (missing)", $name));
                String::new()
            })
        };
    }

    let is_dev = matches!(
        env::var("ENVIRONMENT").as_deref(),
        Ok("dev") | Ok("development")
    );

    let db_write_host = require!("POSTGRES_WRITE_HOST");
    let db_write_port = require!("POSTGRES_WRITE_PORT");
    let db_write_name = require!("POSTGRES_WRITE_NAME");
    let db_write_user = require!("POSTGRES_WRITE_USER");
    let db_write_password = require!("POSTGRES_WRITE_PASSWORD");
    let db_read_host = require!("POSTGRES_READ_HOST");
    let db_read_port = require!("POSTGRES_READ_PORT");
    let db_read_name = require!("POSTGRES_READ_NAME");
    let db_read_user = require!("POSTGRES_READ_USER");
    let db_read_password = require!("POSTGRES_READ_PASSWORD");

    let server_host = require!("HOST");
    let server_port = require!("PORT");

    // Panic with all errors at once
    if !errors.is_empty() {
        panic!(
            "\n\nMissing or invalid environment variables ({} errors):\n{}\n",
            errors.len(),
            errors.join("\n")
        );
    }

    ServerConfig {
        is_dev,

        // Write DB (Primary)
        db_write_host,
        db_write_port,
        db_write_name,
        db_write_user,
        db_write_password,
        db_write_max_connection: env::var("POSTGRES_WRITE_MAX_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100),
        db_write_min_connection: env::var("POSTGRES_WRITE_MIN_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10),

        // Read DB (Replica)
        db_read_host,
        db_read_port,
        db_read_name,
        db_read_user,
        db_read_password,
        db_read_max_connection: env::var("POSTGRES_READ_MAX_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100),
        db_read_min_connection: env::var("POSTGRES_READ_MIN_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10),
        server_host,
        server_port

    }
});

impl ServerConfig {
    // Now you just need to access CONFIG
    pub fn get() -> &'static ServerConfig {
        &CONFIG
    }
}

