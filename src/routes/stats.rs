use crate::db::establish_connection;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub dns_queries: u64,
    pub blocked_queries: u64,
}

pub async fn get_stats() -> Result<Json<Stats>, StatusCode> {
    const TOTAL_NUMBER_OF_QUERIES_ID: i64 = 0;
    const TOTAL_NUMBER_OF_BLOCKED_ID: i64 = 1;
    let conn = establish_connection()?;

    let dns_queries: u64 = conn
        .query_row(
            "SELECT value FROM counters WHERE id = ?",
            [&TOTAL_NUMBER_OF_QUERIES_ID],
            |row| row.get(0),
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let blocked_queries: u64 = conn
        .query_row(
            "SELECT COUNT(*) FROM queries WHERE status = ?",
            [&TOTAL_NUMBER_OF_BLOCKED_ID],
            |row| row.get(0),
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stats = Stats {
        dns_queries,
        blocked_queries,
    };

    Ok(Json(stats))
}
