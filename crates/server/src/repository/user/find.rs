use axum_app_entity::users::{Column as UserColumn, Entity as UserEntity, Model as UserModel};
use axum_app_errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_user_by_id<C>(conn: &C, id: Uuid) -> Result<Option<UserModel>, Errors>
where
    C: ConnectionTrait,
{
    let user = UserEntity::find_by_id(id).one(conn).await?;
    Ok(user)
}

pub async fn repository_find_user_by_email<C>(
    conn: &C,
    email: String,
) -> Result<Option<UserModel>, Errors>
where
    C: ConnectionTrait,
{
    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(email))
        .one(conn)
        .await?;
    Ok(user)
}

pub async fn repository_find_user_by_handle<C>(
    conn: &C,
    handle: String,
) -> Result<Option<UserModel>, Errors>
where
    C: ConnectionTrait,
{
    let user = UserEntity::find()
        .filter(UserColumn::Handle.eq(handle))
        .one(conn)
        .await?;
    Ok(user)
}
