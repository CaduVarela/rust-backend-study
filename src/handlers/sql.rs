use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::db;

#[derive(Deserialize)]
pub struct SqlRequest {
    pub query: String,
}

pub async fn execute_sql(
    pool: web::Data<db::DbPool>,
    sql_request: web::Json<SqlRequest>,
) -> HttpResponse {
    match db::query::execute_query(&pool, &sql_request.query).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::BadRequest().body(format!("Erro: {}", e)),
    }
}
