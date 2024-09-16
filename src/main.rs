use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Stats {
    dns_queries: u64,
    blocked_queries: u64,
}

async fn get_stats() -> Result<Json<Stats>, StatusCode> {
    // Select the database path based on the environment variables
    let db_path = match env::var("RUN_ENV") {
        Ok(env) if env == "production" => "/etc/pihole/pihole-FTL.db",
        _ => "./pihole-FTL.db", // Default sqlite datebase path
    };
    // Connection Pi-hole FTL database
    let conn = Connection::open(db_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Inquire DNS total query count
    let dns_queries: u64 = conn
        .query_row("SELECT COUNT(*) FROM queries", [], |row| row.get(0))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Inquire Blocked DNS query count
    let blocked_queries: u64 = conn
        .query_row("SELECT COUNT(*) FROM queries WHERE status = 1", [], |row| {
            row.get(0)
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let stats = Stats {
        dns_queries,
        blocked_queries,
    };
    Ok(Json(stats)) // Return Contains the success JSON value
}

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/stats", get(get_stats));

    // Run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
