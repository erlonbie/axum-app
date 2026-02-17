use reqwest::Client as HttpClient;
use sea_orm::DatabaseConnection as PostgresqlClient;

#[derive(Clone)]
pub struct AppState {
    pub http_client: HttpClient,
    pub write_db: PostgresqlClient,
    pub read_db: PostgresqlClient,
}
