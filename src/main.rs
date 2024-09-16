use axum::{routing::get, Router};
use routes::{stats::get_stats, top_domains::get_top_domains};

mod routes;

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/stats", get(get_stats))
        .route("/top_domains", get(get_top_domains));

    // Run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
