use axum::Router;
use axum_app_errors::handler_404;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::openapi::ApiDoc;

use crate::api::v1::routes::routes::v1_routes;
use crate::{api::health::routes::health_routes, appstate::AppState};

pub fn api_routes(state: AppState) -> Router<AppState> {
    let mut router = Router::new();

    #[cfg(debug_assertions)]
    {
        router = router.merge(SwaggerUi::new("/docs").url("/swagger.json", ApiDoc::merged()));
    }

    router
        .merge(health_routes())
        .nest("/v1", v1_routes(state))
        .fallback(handler_404)
}
