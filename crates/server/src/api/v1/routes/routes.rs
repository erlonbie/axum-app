use axum::Router;

use crate::{api::v1::routes::user::routes::user_routes, appstate::AppState};


pub fn v1_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(user_routes(state.clone()))
}
