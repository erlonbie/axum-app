use axum::{Json, extract::State, response::IntoResponse};
use axum_app_dto::{
    user::{
        request::create_user::CreateUserRequest,
    },
    validator::json_validator::ValidatedJson,
};
use axum_app_errors::Errors;
use reqwest::StatusCode;

use crate::{appstate::AppState, service::user::create_user::service_create_user};

pub async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_create_user(&state.write_db, payload).await?;

    Ok((StatusCode::CREATED, Json(response)))
}
