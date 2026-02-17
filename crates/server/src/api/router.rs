use axum::Router;

use crate::{api::health::routes::health_routes, appstate::AppState};

pub fn api_routes(state: AppState) -> Router<AppState> {
    let router = Router::new();


    router
        .merge(health_routes())
}
