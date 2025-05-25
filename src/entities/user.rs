use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewUser {
    pub name: String,
}
