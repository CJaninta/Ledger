use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityType {
    INCOME,
    EXPENSE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub user_id: i32,
    pub amount: f64,
    pub activity: String,
    pub activity_type: ActivityType,
}
