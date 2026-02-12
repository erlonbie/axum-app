use axum::{
    routing::get,
    Router,
};

use server::utils::logger::init_tracing;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_tracing();
    // build our application with a single route
    let app = Router::new().route("/", get(handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}
