use axum::{Router, routing::get};

use crate::{
    api::health::health_check::health_check,
    appstate::AppState,
};

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}
