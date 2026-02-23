use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_app_dto::{
    user::{request::update_user::UpdateUserRequest, response::user::UserResponse},
    validator::json_validator::ValidatedJson,
};
use axum_app_errors::Errors;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{appstate::AppState, service::user::update_user::service_update_user};

#[utoipa::path(
    patch,
    path = "/v1/users/{id}",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "User"
)]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_update_user(&state.write_db, id, payload).await?;
    Ok((StatusCode::OK, Json(response)))
}
