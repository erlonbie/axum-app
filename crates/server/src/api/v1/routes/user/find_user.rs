use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_app_dto::user::response::user::UserResponse;
use axum_app_errors::Errors;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    appstate::AppState,
    service::user::find_user::{
        service_find_user_by_email, service_find_user_by_handle, service_find_user_by_id,
    },
};

#[utoipa::path(
    get,
    path = "/v1/users/{id}",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "User"
)]
pub async fn find_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_find_user_by_id(&state.read_db, id).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    get,
    path = "/v1/users/by-email/{email}",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "User"
)]
pub async fn find_user_by_email(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_find_user_by_email(&state.read_db, email).await?;
    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    get,
    path = "/v1/users/by-handle/{handle}",
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "User"
)]
pub async fn find_user_by_handle(
    State(state): State<AppState>,
    Path(handle): Path<String>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_find_user_by_handle(&state.read_db, handle).await?;
    Ok((StatusCode::OK, Json(response)))
}
