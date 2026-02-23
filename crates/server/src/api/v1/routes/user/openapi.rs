use axum_app_dto::user::response::create_user::CreateUserResponse;
use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_user::create_user,
    ),
    components(
        schemas(
            CreateUserResponse
        )
    ),
    tags(
        (name = "User", description = "User endpoints")
    )
)]
pub struct UserApiDoc;
