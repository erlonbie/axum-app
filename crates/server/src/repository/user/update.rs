use axum_app_dto::user::request::update_user::UpdateUserRequest;
use axum_app_entity::users::{ActiveModel as UserActiveModel, Model as UserModel};
use axum_app_errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, IntoActiveModel, Set};

pub async fn repository_update_user<C>(
    conn: &C,
    current_user: UserModel,
    req: UpdateUserRequest,
) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    let mut user: UserActiveModel = current_user.into_active_model();

    if let Some(display_name) = req.display_name {
        user.display_name = Set(display_name);
    }

    if let Some(handle) = req.handle {
        user.handle = Set(handle);
    }

    if let Some(bio) = req.bio {
        user.bio = Set(Some(bio));
    }

    if let Some(profile_image) = req.profile_image {
        user.profile_image = Set(Some(profile_image));
    }

    if let Some(banner_image) = req.banner_image {
        user.banner_image = Set(Some(banner_image));
    }

    let updated_user = user.update(conn).await?;

    Ok(updated_user)
}
