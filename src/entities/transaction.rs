use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub activity: String,
    pub activity_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
