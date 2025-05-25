use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewUser {
    pub name: String,
}
