use std::env;

use sqlx::{Sqlite, Pool, migrate::MigrateDatabase, SqlitePool};
use tokio::sync::OnceCell;
use tracing::info;

static DB_CELL: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

pub async fn db() -> &'static Pool<Sqlite> {
    DB_CELL
        .get_or_init(|| async {
			let db_url = &env::var("DATABASE_URL").unwrap();
			if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
				println!("Creating database {}", db_url);
				match Sqlite::create_database(db_url).await {
					Ok(_) => info!("Create db success"),
					Err(error) => panic!("error: {}", error),
				}
				let db = SqlitePool::connect(&db_url).await.unwrap();
				sqlx::query(include_str!("schema.sql"))
				.execute(&db)
				.await.expect("failed to initialize database");
				db
			} else {
				info!("Database already exists");
				SqlitePool::connect(&db_url).await.unwrap()
			}
        })
        .await
}
