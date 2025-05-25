use std::sync::Arc;
use axum::{ extract::{ Query, State }, http::StatusCode, response::IntoResponse, Json };
use crate::{
    models::user::{ ByIDRequest, CreateUserRequest, UpdateUserRequest },
    usecases::user::{ UserUseCase, UserUseCaseImpl },
};

pub async fn create_user(
    State(usecase): State<Arc<UserUseCaseImpl>>,
    Json(body): Json<CreateUserRequest>
) -> impl IntoResponse {
    match usecase.create_user(body).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => e.error().into_response(),
    }
}

pub async fn get_user_by_id(
    State(usecase): State<Arc<UserUseCaseImpl>>,
    Query(params): Query<ByIDRequest>
) -> impl IntoResponse {
    match usecase.get_user_by_id(params.id.unwrap()).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => e.error().into_response(),
    }
}

pub async fn update_user(
    State(usecase): State<Arc<UserUseCaseImpl>>,
    Json(body): Json<UpdateUserRequest>
) -> impl IntoResponse {
    match usecase.update_user(body).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => e.error().into_response(),
    }
}

pub async fn delete_user(
    State(usecase): State<Arc<UserUseCaseImpl>>,
    Query(params): Query<ByIDRequest>
) -> impl IntoResponse {
    match usecase.delete_user(params.id.unwrap()).await {
        Ok(_) => (StatusCode::OK, Json(())).into_response(),
        Err(e) => e.error().into_response(),
    }
}
