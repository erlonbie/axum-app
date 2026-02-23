use axum_app_entity::users::{ActiveModel as UserActiveModel, Model as UserModel};
use axum_app_errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

pub async fn repository_create_user<C>(
    conn: &C,
    email: String,
    handle: String,
    display_name: String,
    password: String,
) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    let new_user = UserActiveModel {
        id: Default::default(),
        display_name: Set(display_name),
        handle: Set(handle),
        bio: Set(None),
        email: Set(email),
        password: Set(Some(password)),
        verified_at: Set(None),
        profile_image: Set(None),
        banner_image: Set(None),
        totp_secret: Set(None),
        totp_enabled_at: Set(None),
        totp_backup_codes: Set(None),
        created_at: Default::default(),
    };

    let user = new_user.insert(conn).await?;

    Ok(user)
}
