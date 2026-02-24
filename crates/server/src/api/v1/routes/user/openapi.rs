use axum_app_dto::user::{
    request::update_user::UpdateUserRequest,
    response::{create_user::CreateUserResponse, user::UserResponse},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_user::create_user,
        super::find_user::find_user_by_id,
        super::find_user::find_user_by_email,
        super::find_user::find_user_by_handle,
        super::update_user::update_user,
        super::delete_user::delete_user,
    ),
    components(
        schemas(
            CreateUserResponse,
            UserResponse,
            UpdateUserRequest
        )
    ),
    tags(
        (name = "User", description = "User endpoints")
    )
)]
pub struct UserApiDoc;
