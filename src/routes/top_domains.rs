use crate::db::establish_connection;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DomainStats {
    pub domain: String,
    pub count: u64,
}

pub async fn get_top_domains() -> Result<Json<Vec<DomainStats>>, StatusCode> {
    let conn = establish_connection()?;

    let mut stmt = conn.prepare(
        "SELECT domain, COUNT(domain) as count FROM queries GROUP BY domain ORDER BY count DESC LIMIT 10"
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let domain_stats = stmt
        .query_map([], |row| {
            Ok(DomainStats {
                domain: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .collect::<Result<Vec<DomainStats>, _>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(domain_stats))
}
