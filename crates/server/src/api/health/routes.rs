use axum::{Router, routing::get};

use crate::api::health::health_check::health_check;

pub fn health_routes() -> axum::Router<crate::appstate::AppState> {
    Router::new().route("/health", get(health_check))
}
