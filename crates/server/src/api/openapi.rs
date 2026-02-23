use axum_app_errors::ErrorResponse;
use utoipa::OpenApi;

use crate::api::{health::openapi::HealthApiDoc, v1::openapi::V1ApiDoc};

#[derive(OpenApi)]
#[openapi(components(schemas(ErrorResponse,)))]
pub struct ApiDoc;

impl ApiDoc {
    pub fn merged() -> utoipa::openapi::OpenApi {
        let mut openapi = Self::openapi();
        openapi.merge(HealthApiDoc::openapi());
        openapi.merge(V1ApiDoc::merged());
        openapi
    }
}
