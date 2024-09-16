use axum::{http::StatusCode, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub dns_queries: u64,
    pub blocked_queries: u64,
}

pub async fn get_stats() -> Result<Json<Stats>, StatusCode> {
    let db_path = match env::var("RUN_ENV") {
        Ok(env) if env == "production" => "/etc/pihole/pihole-FTL.db",
        _ => "./pihole-FTL.db",
    };

    let conn = Connection::open(db_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dns_queries: u64 = conn
        .query_row("SELECT COUNT(*) FROM queries", [], |row| row.get(0))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blocked_queries: u64 = conn
        .query_row("SELECT COUNT(*) FROM queries WHERE status = 1", [], |row| {
            row.get(0)
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stats = Stats {
        dns_queries,
        blocked_queries,
    };

    Ok(Json(stats))
}
