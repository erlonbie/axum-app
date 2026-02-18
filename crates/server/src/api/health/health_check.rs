use axum::{http::StatusCode, response::IntoResponse};
use axum_app_errors::Errors;

#[utoipa::path(
    get,
    path = "/health-check",
    responses(
        (status = 204, description = "Service is healthy and running"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Health"
)]
pub async fn health_check() -> Result<impl IntoResponse, Errors> {
    Ok(StatusCode::NO_CONTENT)
}
