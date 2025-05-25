use std::sync::Arc;
use axum::{ http::StatusCode, response::IntoResponse, Json };
use crate::{ models::user::CreateUserRequest, usecases::user::{ UserUseCase, UserUseCaseImpl } };

pub async fn create_user(
    Json(body): Json<CreateUserRequest>,
    usecase: Arc<UserUseCaseImpl>
) -> impl IntoResponse {
    match usecase.create_user(body).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => e.error().into_response(),
    }
}
