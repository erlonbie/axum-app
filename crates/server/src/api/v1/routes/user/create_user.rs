use axum::{Json, extract::State, response::IntoResponse};
use axum_app_dto::{
    user::{
        request::create_user::CreateUserRequest,
        response::create_user::CreateUserResponse,
    },
    validator::json_validator::ValidatedJson,
};
use axum_app_errors::Errors;
use reqwest::StatusCode;

use crate::{appstate::AppState, service::user::create_user::service_create_user};

#[utoipa::path(
    post,
    path = "/v1/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = CreateUserResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 409, description = "Conflict - User with this email or handle already exists"),
        (status = 500, description = "Internal Server Error - Database or Redis error"),
        (status = 502, description = "Bad Gateway - Worker service request failed or returned invalid response"),
        (status = 503, description = "Service Unavailable - Worker service connection failed"),
    ),
    tag = "User"
)]
pub async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_create_user(&state.write_db, payload).await?;

    Ok((StatusCode::CREATED, Json(response)))
}
