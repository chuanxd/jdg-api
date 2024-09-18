use axum::http::StatusCode;
use rusqlite::{Connection, Result};
use std::env;

pub fn establish_connection() -> Result<Connection, StatusCode> {
    let db_path = match env::var("RUN_ENV") {
        Ok(env) if env == "production" => "/etc/pihole/pihole-FTL.db",
        _ => "./pihole-FTL.db",
    };

    Connection::open(db_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
