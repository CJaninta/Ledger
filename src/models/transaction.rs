use serde::{ Deserialize, Serialize };
use chrono::NaiveDateTime;
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum ActivityType {
    INCOME,
    EXPENSE,
}

#[derive(Debug, Deserialize)]
pub struct TransactionRequest {
    pub user_id: i32,
    pub amount: f64,
    pub activity: String,
    pub activity_type: ActivityType,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: i32,
    pub activity: String,
    pub activity_type: ActivityType,
    pub amount: f64,
    pub created_at: NaiveDateTime,
}
