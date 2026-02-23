use axum_app_errors::{errors::ServiceResult, Errors};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::repository::user::{repository_delete_user, repository_find_user_by_id};

pub async fn service_delete_user(conn: &DatabaseConnection, id: Uuid) -> ServiceResult<()> {
    let current_user = repository_find_user_by_id(conn, id).await?;
    let current_user = match current_user {
        Some(user) => user,
        None => return Err(Errors::UserNotFound),
    };

    repository_delete_user(conn, current_user).await?;
    Ok(())
}
