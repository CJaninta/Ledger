use std::{ sync::Arc };
use mockall::automock;
use crate::{
    models::{
        error::{ APIError, IntoErrorResponse },
        transaction::{ TransactionRequest, TransactionResponse, ActivityType },
    },
    repositories::transaction::{ TransactionRepo, TransactionRepoImpl },
};

#[async_trait::async_trait]
#[automock]
pub trait TransactionUseCase {
    fn new(transaction_repo: Arc<TransactionRepoImpl>) -> Arc<Self>;
    async fn create_transaction(
        &self,
        request: TransactionRequest
    ) -> Result<TransactionResponse, Box<dyn IntoErrorResponse>>;
}

pub struct TransactionUseCaseImpl {
    transaction_repo: Arc<TransactionRepoImpl>,
}

#[async_trait::async_trait]
impl TransactionUseCase for TransactionUseCaseImpl {
    fn new(transaction_repo: Arc<TransactionRepoImpl>) -> Arc<Self> {
        Arc::new(Self { transaction_repo })
    }

    async fn create_transaction(
        &self,
        request: TransactionRequest
    ) -> Result<TransactionResponse, Box<dyn IntoErrorResponse>> {
        if request.amount <= 0.0 {
            return Err(
                Box::new(
                    APIError::CreateTransactionError(
                        "Transaction amount must be greater than zero".to_string()
                    )
                )
            );
        }
        if request.activity.is_empty() {
            return Err(
                Box::new(APIError::CreateTransactionError("Activity cannot be empty".to_string()))
            );
        }
        if request.user_id <= 0 {
            return Err(
                Box::new(
                    APIError::CreateTransactionError(
                        "User ID must be greater than zero".to_string()
                    )
                )
            );
        }

        let transaction = self.transaction_repo.create_transaction(
            request.user_id,
            request.amount,
            request.activity.as_str(),
            request.activity_type
        ).await;

        match transaction {
            Ok(tx) => {
                let activity_type_str = match tx.activity_type.as_str() {
                    "INCOME" => ActivityType::INCOME,
                    "EXPENSE" => ActivityType::EXPENSE,
                    _ => panic!("Unknown activity type"),
                };

                Ok(TransactionResponse {
                    id: tx.id,
                    activity: tx.activity,
                    activity_type: activity_type_str,
                    amount: tx.amount,
                    created_at: tx.created_at,
                })
            }
            Err(e) =>
                Err(
                    Box::new(
                        APIError::CreateTransactionError(
                            format!("Failed to add transaction, err: {}", e)
                        )
                    )
                ),
        }
    }
}
