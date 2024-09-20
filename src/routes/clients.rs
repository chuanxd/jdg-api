use crate::db::establish_connection;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Clients {
    pub id: u64,
    pub ip: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopClients {
    pub id: u64,
    pub name: String,
    pub num_queries: u64,
    pub ip: String,
}

pub async fn get_clients() -> Result<Json<Vec<Clients>>, StatusCode> {
    let conn = establish_connection()?;

    let mut stmt = conn
        .prepare("SELECT id, ip, name FROM client_by_id LIMIT 1000 OFFSET 0")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let clients = stmt
        .query_map([], |row| {
            Ok(Clients {
                id: row.get(0)?,
                ip: row.get(1)?,
                name: row.get(2)?,
            })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .collect::<Result<Vec<Clients>, _>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(clients))
}

pub async fn get_top_clients() -> Result<Json<Vec<TopClients>>, StatusCode> {
    let conn = establish_connection()?;

    let mut stmt = conn
        .prepare(
            "SELECT n.id, na.name, n.numQueries, na.ip
            FROM network AS n
            JOIN network_addresses AS na ON n.id = na.network_id
            WHERE na.name IS NOT NULL AND na.name != ''
            ORDER BY n.numQueries DESC;",
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let clients = stmt
        .query_map([], |row| {
            Ok(TopClients {
                id: row.get(0)?,
                name: row.get(1)?,
                num_queries: row.get(2)?,
                ip: row.get(3)?,
            })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .collect::<Result<Vec<TopClients>, _>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(clients))
}
