use axum::http::{HeaderName, HeaderValue};
use dotenvy::dotenv;
use tracing::warn;
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

    pub cors_allowed_origins: Vec<HeaderValue>,
    pub cors_allowed_headers: Vec<HeaderName>,
    pub cors_max_age: Option<u64>,
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

    let cors_origins: Vec<HeaderValue> = match env::var("CORS_ALLOWED_ORIGINS").ok() {
        Some(origins) => origins
            .split(',')
            .filter_map(|s| {
                let trimmed_s = s.trim();
                if trimmed_s.is_empty() {
                    warn!("Empty origin found in CORS_ALLOWED_ORIGINS.");
                    None
                } else {
                    HeaderValue::from_str(trimmed_s).ok().or_else(|| {
                        warn!(
                            "Invalid origin '{}' found in CORS_ALLOWED_ORIGINS.",
                            trimmed_s
                        );
                        None
                    })
                }
            })
            .collect(),
        None => {
            vec![]
        }
    };

    let cors_headers: Vec<HeaderName> = match env::var("CORS_ALLOWED_HEADERS").ok() {
        Some(headers) => headers
            .split(',')
            .filter_map(|s| {
                let trimmed_s = s.trim();
                if trimmed_s.is_empty() {
                    warn!("Empty header name found in CORS_ALLOWED_HEADERS.");
                    None
                } else {
                    HeaderName::from_bytes(trimmed_s.as_bytes())
                        .ok()
                        .or_else(|| {
                            warn!(
                                "Invalid header name '{}' found in CORS_ALLOWED_HEADERS.",
                                trimmed_s
                            );
                            None
                        })
                }
            })
            .collect(),
        None => {
            vec![]
        }
    };

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
        server_port,

        cors_allowed_origins: cors_origins,
        cors_allowed_headers: cors_headers,
        cors_max_age: env::var("CORS_MAX_AGE").ok().and_then(|v| v.parse().ok()),

    }
});

impl ServerConfig {
    // Now you just need to access CONFIG
    pub fn get() -> &'static ServerConfig {
        &CONFIG
    }
}

