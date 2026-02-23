use axum::{Router, routing::post};

use crate::{api::v1::routes::user::create_user::create_user, appstate::AppState};

pub fn user_routes(state: AppState) -> Router<AppState> {
    // Public routes (no authentication required)
    println!("State: {:?}", state);
    let public_routes = Router::new().route("/users", post(create_user));
    public_routes
}
