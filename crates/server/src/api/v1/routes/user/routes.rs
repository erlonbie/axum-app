use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    api::v1::routes::user::{
        create_user::create_user,
        delete_user::delete_user,
        find_user::{find_user_by_email, find_user_by_handle, find_user_by_id},
        update_user::update_user,
    },
    appstate::AppState,
};

pub fn user_routes(state: AppState) -> Router<AppState> {
    // Public routes (no authentication required)
    println!("State: {:?}", state);
    let public_routes = Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", get(find_user_by_id))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
        .route("/users/by-email/{email}", get(find_user_by_email))
        .route("/users/by-handle/{handle}", get(find_user_by_handle));
    public_routes
}
