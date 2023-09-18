#[derive(sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub email: String,
    pub created: chrono::NaiveDateTime,
    pub modified: chrono::NaiveDateTime,
    pub banned: bool,
    pub score: u64,
    pub hash: [u8; 32],
}
