use utoipa::OpenApi;

use crate::api::v1::routes::user::openapi::UserApiDoc;


#[derive(OpenApi)]
#[openapi()]
pub struct V1ApiDoc;

impl V1ApiDoc {
    pub fn merged() -> utoipa::openapi::OpenApi {
        let mut openapi = Self::openapi();
        openapi.merge(UserApiDoc::openapi());
        openapi
    }
}
