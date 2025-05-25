use serde::{ Deserialize, Serialize };
use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub id: u64,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ByIDRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserTransactionResponse {
    pub user_name: String,
    pub total_income: f32,
    pub total_expense: f32,
}
