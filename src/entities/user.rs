use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
