use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod db;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = db::connect().await.expect("Falha ao conectar ao banco");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .configure(routes::configure)
    })
    .bind("127.0.0.1:4444")?
    .run()
    .await
}
