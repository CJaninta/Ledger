use std::sync::Arc;
use axum::{ extract::{ State, Json }, http::StatusCode, response::IntoResponse };
use crate::{
    models::transaction::{ TransactionRequest },
    usecases::transaction::{ TransactionUseCase, TransactionUseCaseImpl },
};

pub async fn create_transaction(
    State(usecase): State<Arc<TransactionUseCaseImpl>>,
    Json(body): Json<TransactionRequest>
) -> impl IntoResponse {
    match usecase.create_transaction(body).await {
        Ok(transaction) => (StatusCode::CREATED, Json(transaction)).into_response(),
        Err(e) => e.error().into_response(),
    }
}
