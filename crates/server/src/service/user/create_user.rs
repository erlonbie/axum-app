use axum_app_dto::user::{
    request::create_user::CreateUserRequest, response::create_user::CreateUserResponse,
};
use axum_app_errors::errors::ServiceResult;
use sea_orm::{DatabaseConnection, TransactionTrait};

use crate::repository::user::repository_create_user;

pub async fn service_create_user(
    conn: &DatabaseConnection,
    payload: CreateUserRequest,
) -> ServiceResult<CreateUserResponse> {
    let txn = conn.begin().await?;

    // // Check if user already exists by email
    // let existing_user_by_email = repository_find_user_by_email(&txn, payload.email.clone()).await?;
    // if existing_user_by_email.is_some() {
    //     return Err(Errors::UserEmailAlreadyExists);
    // }


    // // Check if user already exists by handle
    // let existing_user_by_handle =
    //     repository_find_user_by_handle(&txn, payload.handle.clone()).await?;
    // if existing_user_by_handle.is_some() {
    //     return Err(Errors::UserHandleAlreadyExists);
    // }
    
    // Create user in database
    let _user = repository_create_user(
        &txn,
        payload.email.clone(),
        payload.handle.clone(),
        payload.display_name,
        payload.password,
    )
    .await?;

    txn.commit().await?;

    Ok(CreateUserResponse {
        message: "User created successfully. Please check your email to verify your account."
            .to_string(),
    })
}
