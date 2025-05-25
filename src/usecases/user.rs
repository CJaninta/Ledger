use std::{ sync::Arc };
use crate::{
    models::{
        error::{ APIError, IntoErrorResponse },
        user::{ CreateUserRequest, CreateUserResponse, UpdateUserRequest, UserResponse },
    },
    repositories::user::{ UserRepo, UserRepoImpl },
};

#[async_trait::async_trait]
pub trait UserUseCase {
    fn new(user_repo: Arc<UserRepoImpl>) -> Arc<Self>;
    async fn create_user(
        &self,
        request: CreateUserRequest
    ) -> Result<CreateUserResponse, Box<dyn IntoErrorResponse>>;
    fn update_user(
        &self,
        user_id: u64,
        request: UpdateUserRequest
    ) -> Result<UserResponse, Box<dyn IntoErrorResponse>>;
    fn delete_user(&self, user_id: u64) -> Result<(), Box<dyn IntoErrorResponse>>;
    fn get_user(&self, user_id: u64) -> Result<UserResponse, Box<dyn IntoErrorResponse>>;
}

pub struct UserUseCaseImpl {
    user_repo: Arc<UserRepoImpl>,
}

#[async_trait::async_trait]
impl UserUseCase for UserUseCaseImpl {
    fn new(user_repo: Arc<UserRepoImpl>) -> Arc<Self> {
        Arc::new(UserUseCaseImpl {
            user_repo,
        })
    }

    async fn create_user(
        &self,
        request: CreateUserRequest
    ) -> Result<CreateUserResponse, Box<dyn IntoErrorResponse>> {
        if request.name.is_empty() {
            return Err(Box::new(APIError::CreateUserError("Username cannot be empty".to_string())));
        }

        let result_create = self.user_repo.create_user(request.name.as_str()).await;
        match result_create {
            Ok(user_id) => {
                let result = CreateUserResponse {
                    id: user_id,
                };
                Ok(result)
            }
            Err(e) => {
                Err(
                    Box::new(
                        APIError::CreateUserError(format!("Failed to create user, err: {}", e))
                    )
                )
            }
        }
    }

    fn update_user(
        &self,
        user_id: u64,
        request: UpdateUserRequest
    ) -> Result<UserResponse, Box<dyn IntoErrorResponse>> {
        // if user_id == 0 {
        //     return Err(Box::new(APIError::CreateUserError("Invalid user ID".to_string())));
        // }

        // if request.name.is_empty() {
        //     return Err(Box::new(APIError::CreateUserError("Username cannot be empty".to_string())));
        // }

        Ok(UserResponse {
            id: user_id, // Placeholder for user ID
            name: request.name,
            created_at: chrono::NaiveDateTime
                ::parse_from_str("2023-10-01T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            updated_at: chrono::NaiveDateTime
                ::parse_from_str("2023-10-01T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
        })
    }

    fn delete_user(&self, user_id: u64) -> Result<(), Box<dyn IntoErrorResponse>> {
        if user_id == 0 {
            return Err(Box::new(APIError::DeleteUserError("Invalid user ID".to_string())));
        }

        Ok(())
    }

    fn get_user(&self, user_id: u64) -> Result<UserResponse, Box<dyn IntoErrorResponse>> {
        if user_id == 0 {
            return Err(Box::new(APIError::GetUserError("Invalid user ID".to_string())));
        }

        Ok(UserResponse {
            id: user_id,
            name: "Sample User".to_string(),
            created_at: chrono::NaiveDateTime
                ::parse_from_str("2023-10-01T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
            updated_at: chrono::NaiveDateTime
                ::parse_from_str("2023-10-01T00:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
        })
    }
}
