use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use axum_app_errors::Errors;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{appstate::AppState, service::user::delete_user::service_delete_user};

#[utoipa::path(
    delete,
    path = "/v1/users/{id}",
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "User"
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Errors> {
    service_delete_user(&state.write_db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
