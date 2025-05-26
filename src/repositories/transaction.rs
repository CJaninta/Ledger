use std::sync::Arc;
use mockall::automock;
use sqlx::{ MySqlPool };
use crate::{ entities::transaction::Transaction, models::transaction::ActivityType };

#[async_trait::async_trait]
#[automock]
pub trait TransactionRepo {
    fn new(pool: MySqlPool) -> Arc<Self>;
    async fn create_transaction(
        &self,
        user_id: i32,
        amount: f64,
        activity: &str,
        activity_type: ActivityType
    ) -> Result<Transaction, sqlx::Error>;
}

pub struct TransactionRepoImpl {
    pub pool: MySqlPool,
}

#[async_trait::async_trait]
impl TransactionRepo for TransactionRepoImpl {
    fn new(pool: MySqlPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    async fn create_transaction(
        &self,
        user_id: i32,
        amount: f64,
        activity: &str,
        activity_type: ActivityType
    ) -> Result<Transaction, sqlx::Error> {
        let activity_type_str = match activity_type {
            ActivityType::INCOME => "INCOME",
            ActivityType::EXPENSE => "EXPENSE",
        };
        let result = sqlx
            ::query(
                r#"
            INSERT INTO transactions (user_id, amount, activity, activity_type)
            VALUES (?, ?, ?, ?)
            "#
            )
            .bind(user_id)
            .bind(amount)
            .bind(activity)
            .bind(activity_type_str)
            .execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let added_transaction = sqlx
            ::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = ?")
            .bind(result.last_insert_id())
            .fetch_one(&self.pool).await?;

        Ok(added_transaction)
    }
}
