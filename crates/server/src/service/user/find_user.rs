use axum_app_dto::user::response::user::UserResponse;
use axum_app_entity::users::Model as UserModel;
use axum_app_errors::{errors::ServiceResult, Errors};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::repository::user::{
    repository_find_user_by_email, repository_find_user_by_handle, repository_find_user_by_id,
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

pub async fn service_find_user_by_id(
    conn: &DatabaseConnection,
    id: Uuid,
) -> ServiceResult<UserResponse> {
    let user = repository_find_user_by_id(conn, id).await?;
    match user {
        Some(user) => Ok(into_response(user)),
        None => Err(Errors::UserNotFound),
    }
}

pub async fn service_find_user_by_email(
    conn: &DatabaseConnection,
    email: String,
) -> ServiceResult<UserResponse> {
    let user = repository_find_user_by_email(conn, email).await?;
    match user {
        Some(user) => Ok(into_response(user)),
        None => Err(Errors::UserNotFound),
    }
}

pub async fn service_find_user_by_handle(
    conn: &DatabaseConnection,
    handle: String,
) -> ServiceResult<UserResponse> {
    let user = repository_find_user_by_handle(conn, handle).await?;
    match user {
        Some(user) => Ok(into_response(user)),
        None => Err(Errors::UserNotFound),
    }
}
