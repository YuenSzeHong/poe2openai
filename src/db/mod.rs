pub mod interactions;

use sqlx::{Pool, SqlitePool, Postgres, MySql, Sqlite};

#[derive(Clone)]
pub struct DbPool {
    pub sqlite: Option<Pool<Sqlite>>,
    pub postgres: Option<Pool<Postgres>>,
    pub mysql: Option<Pool<MySql>>,
}

pub async fn create_db_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let sqlite_pool = if database_url.starts_with("sqlite") {
        println!("Using SQLite");
        let pool = SqlitePool::connect(&database_url).await?;
        Some(pool)
    } else {
        None
    };

    let postgres_pool = if database_url.starts_with("postgres") {
        println!("Using Postgres");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        Some(pool)
    } else {
        None
    };

     let mysql_pool = if database_url.starts_with("mysql") {
        println!("Using MySQL");
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        Some(pool)
    } else {
        None
    };

    Ok(DbPool {
        sqlite: sqlite_pool,
        postgres: postgres_pool,
        mysql: mysql_pool,
    })
}
