use std::{ sync::Arc };
use mockall::automock;
use crate::{
    models::{
        error::{ APIError, IntoErrorResponse },
        user::{ CreateUserRequest, CreateUserResponse, UpdateUserRequest, UserResponse },
    },
    repositories::user::{ UserRepo, UserRepoImpl },
};

#[async_trait::async_trait]
#[automock]
pub trait UserUseCase {
    fn new(user_repo: Arc<UserRepoImpl>) -> Arc<Self>;
    async fn create_user(
        &self,
        request: CreateUserRequest
    ) -> Result<CreateUserResponse, Box<dyn IntoErrorResponse>>;
    async fn update_user(
        &self,
        request: UpdateUserRequest
    ) -> Result<UserResponse, Box<dyn IntoErrorResponse>>;
    async fn delete_user(&self, user_id: i32) -> Result<(), Box<dyn IntoErrorResponse>>;
    async fn get_user_by_id(
        &self,
        user_id: i32
    ) -> Result<UserResponse, Box<dyn IntoErrorResponse>>;
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

    async fn get_user_by_id(
        &self,
        user_id: i32
    ) -> Result<UserResponse, Box<dyn IntoErrorResponse>> {
        if user_id == 0 {
            return Err(Box::new(APIError::GetUserError("Invalid user ID".to_string())));
        }

        let result_query = self.user_repo.get_user_by_id(user_id).await;
        match result_query {
            Ok(user) => {
                Ok(UserResponse {
                    id: user.id,
                    name: user.name,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
            }
            Err(e) => {
                Err(Box::new(APIError::GetUserError(format!("Failed to get user, err: {}", e))))
            }
        }
    }

    async fn update_user(
        &self,
        request: UpdateUserRequest
    ) -> Result<UserResponse, Box<dyn IntoErrorResponse>> {
        if request.id == 0 {
            return Err(Box::new(APIError::UpdateUserError("Invalid user ID".to_string())));
        }

        let result_update = self.user_repo.update_user(request.id, request.name.as_str()).await;
        match result_update {
            Ok(user) => {
                Ok(UserResponse {
                    id: user.id,
                    name: user.name,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
            }
            Err(e) => {
                Err(
                    Box::new(
                        APIError::UpdateUserError(format!("Failed to update user, err: {}", e))
                    )
                )
            }
        }
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), Box<dyn IntoErrorResponse>> {
        if user_id == 0 {
            return Err(Box::new(APIError::DeleteUserError("Invalid user ID".to_string())));
        }

        let result_delete = self.user_repo.delete_user(user_id).await;
        match result_delete {
            Ok(_) => Ok(()),
            Err(e) =>
                Err(
                    Box::new(
                        APIError::DeleteUserError(format!("Failed to delete user, err: {}", e))
                    )
                ),
        }
    }
}
