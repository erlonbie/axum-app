use axum_app_dto::user::{request::update_user::UpdateUserRequest, response::user::UserResponse};
use axum_app_entity::users::Model as UserModel;
use axum_app_errors::{errors::ServiceResult, Errors};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::repository::user::{
    repository_find_user_by_handle, repository_find_user_by_id, repository_update_user,
};

fn into_response(user: UserModel) -> UserResponse {
    UserResponse {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        bio: user.bio,
        email: user.email,
        verified_at: user.verified_at,
        profile_image: user.profile_image,
        banner_image: user.banner_image,
        totp_enabled: user.totp_enabled_at.is_some(),
        created_at: user.created_at,
    }
}

pub async fn service_update_user(
    conn: &DatabaseConnection,
    id: Uuid,
    req: UpdateUserRequest,
) -> ServiceResult<UserResponse> {
    let current_user = repository_find_user_by_id(conn, id).await?;
    let current_user = match current_user {
        Some(user) => user,
        None => return Err(Errors::UserNotFound),
    };

    if let Some(new_handle) = &req.handle {
        if new_handle != &current_user.handle {
            if (repository_find_user_by_handle(conn, new_handle.clone()).await?).is_some() {
                return Err(Errors::UserHandleAlreadyExists);
            }
        }
    }

    let updated_user = repository_update_user(conn, current_user, req).await?;
    Ok(into_response(updated_user))
}
