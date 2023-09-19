use std::env;

use sqlx::{MySql, Pool};
use tokio::sync::OnceCell;

static DB_CELL: OnceCell<Pool<MySql>> = OnceCell::const_new();

pub async fn db() -> &'static Pool<MySql> {
    DB_CELL
        .get_or_init(|| async {
            sqlx::MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
                .await
                .unwrap()
        })
        .await
}
