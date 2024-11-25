use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("", web::get().to(|| async { "Hello World!" }))
            .route("/query", web::post().to(handlers::sql::execute_sql)),
    );
}
