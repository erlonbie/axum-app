use axum_app_entity::users::Model as UserModel;
use axum_app_errors::Errors;
use sea_orm::{ConnectionTrait, ModelTrait};

pub async fn repository_delete_user<C>(conn: &C, user: UserModel) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    user.delete(conn).await?;
    Ok(())
}
