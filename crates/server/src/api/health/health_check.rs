use axum::{http::StatusCode, response::IntoResponse};
use axum_app_errors::Errors;

pub async fn health_check() -> Result<impl IntoResponse, Errors> {
    Ok(StatusCode::NO_CONTENT)
}
