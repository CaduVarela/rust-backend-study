use sqlx::{sqlite::SqlitePool, Pool};

pub type DbPool = Pool<sqlx::Sqlite>;

pub mod query;

pub async fn connect() -> Result<DbPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL não configurado");
    SqlitePool::connect(&database_url).await
}
