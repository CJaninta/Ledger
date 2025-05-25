use std::sync::Arc;
use mockall::automock;
use sqlx::{ MySqlPool };
use sqlx::mysql::MySqlQueryResult;

#[async_trait::async_trait]
#[automock]
pub trait UserRepo {
    fn new(pool: MySqlPool) -> Arc<Self>;
    async fn create_user(&self, name: &str) -> Result<u64, sqlx::Error>;
}

pub struct UserRepoImpl {
    pub pool: MySqlPool,
}

#[async_trait::async_trait]
impl UserRepo for UserRepoImpl {
    fn new(pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    async fn create_user(&self, name: &str) -> Result<u64, sqlx::Error> {
        // Insert the user
        let result: MySqlQueryResult = sqlx
            ::query("INSERT INTO users (name) VALUES (?)")
            .bind(name)
            .execute(&self.pool).await?;

        // Get the last insert ID
        let last_id = result.last_insert_id();

        Ok(last_id)
    }

    // pub async fn get_user_by_id(&self, user_id: u64) -> Result<User, Box<dyn std::error::Error>> {
    //     let user = sqlx
    //         ::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    //         .bind(user_id)
    //         .fetch_one(&self.pool).await?;

    //     Ok(user)
    // }
}
