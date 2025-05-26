use std::sync::Arc;
use mockall::automock;
use sqlx::{ MySqlPool };
use crate::entities::user::User;

#[async_trait::async_trait]
#[automock]
pub trait UserRepo {
    fn new(pool: MySqlPool) -> Arc<Self>;
    async fn create_user(&self, name: &str) -> Result<i32, sqlx::Error>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<User, sqlx::Error>;
    async fn update_user(&self, user_id: i32, name: &str) -> Result<User, sqlx::Error>;
    async fn delete_user(&self, user_id: i32) -> Result<(), sqlx::Error>;
}

pub struct UserRepoImpl {
    pub pool: MySqlPool,
}

#[async_trait::async_trait]
impl UserRepo for UserRepoImpl {
    fn new(pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    async fn create_user(&self, name: &str) -> Result<i32, sqlx::Error> {
        // Insert the user
        let result = sqlx
            ::query("INSERT INTO users (name) VALUES (?)")
            .bind(name)
            .execute(&self.pool).await?;

        // Get the last insert ID
        let last_id = result.last_insert_id();

        Ok(last_id as i32)
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User, sqlx::Error> {
        let user = sqlx
            ::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&self.pool).await?;

        Ok(user)
    }

    async fn update_user(&self, user_id: i32, name: &str) -> Result<User, sqlx::Error> {
        let updated_user = sqlx
            ::query("UPDATE users SET name = ? WHERE id = ?")
            .bind(name)
            .bind(user_id)
            .execute(&self.pool).await?;

        if updated_user.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let updated_user = sqlx
            ::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&self.pool).await?;

        Ok(updated_user)
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = ?").bind(user_id).execute(&self.pool).await?;

        Ok(())
    }
}
