use std::sync::Arc;
use crate::{
    models::{ user::{ CreateUserRequest, UserResponse }, error::{ APIError, IntoErrorResponse } },
};

pub struct UserUseCase;

impl UserUseCase {
    pub fn new() -> Arc<Self> {
        Arc::new(UserUseCase)
    }

    pub fn create_user(
        &self,
        request: CreateUserRequest
    ) -> Result<UserResponse, Box<dyn IntoErrorResponse>> {
        if request.name.is_empty() {
            return Err(Box::new(APIError::CreateUserError("Username cannot be empty".to_string())));
        }

        Ok(UserResponse {
            id: 1,
            name: request.name,
            created_at: "2023-10-01T00:00:00Z".to_string(),
            updated_at: "2023-10-01T00:00:00Z".to_string(),
        })
    }
}
