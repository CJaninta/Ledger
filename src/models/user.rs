use serde::{ Deserialize, Serialize };
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ByIDRequest {
    pub id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct GetUserTransactionResponse {
    pub user_name: String,
    pub total_income: f32,
    pub total_expense: f32,
}
