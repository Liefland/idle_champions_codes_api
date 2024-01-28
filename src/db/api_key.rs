use rocket_db_pools::sqlx;
use sqlx::PgConnection;

pub async fn is_valid_api_key(conn: &mut PgConnection, api_key: &str) -> Option<i32> {
    match sqlx::query!("select id from api_keys where api_key = $1", api_key)
        .fetch_one(conn)
        .await
    {
        Ok(row) => Some(row.id),
        Err(_) => None,
    }
}
