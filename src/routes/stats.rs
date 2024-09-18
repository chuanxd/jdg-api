use crate::db::establish_connection;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub dns_queries: u64,
    pub blocked_queries: u64,
}

pub async fn get_stats() -> Result<Json<Stats>, StatusCode> {
    let conn = establish_connection()?;

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
