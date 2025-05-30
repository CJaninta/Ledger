use axum::{ http::StatusCode, response::{ IntoResponse, Response }, Json };
use serde_json::json;

#[derive(Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub status_code: StatusCode,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(
                json!(
                {
                    "error": self.error
                }
            )
            ),
        ).into_response()
    }
}

pub trait IntoErrorResponse {
    fn error(&self) -> ErrorResponse;
}

pub enum APIError {
    UserAlreadyExists(String),
    CreateUserError(String),
    UpdateUserError(String),
    DeleteUserError(String),
    GetUserError(String),
    CreateTransactionError(String),
    UnExpectedError(String),
}

impl IntoErrorResponse for APIError {
    fn error(&self) -> ErrorResponse {
        match self {
            Self::UserAlreadyExists(message) =>
                ErrorResponse {
                    error: format!("user is already exists: {}", message),
                    status_code: StatusCode::BAD_REQUEST,
                },
            Self::CreateUserError(message) =>
                ErrorResponse {
                    error: format!("create user error: {}", message),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                },
            Self::UpdateUserError(message) =>
                ErrorResponse {
                    error: format!("update user error: {}", message),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                },
            Self::DeleteUserError(message) =>
                ErrorResponse {
                    error: format!("delete user error: {}", message),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                },
            Self::GetUserError(message) =>
                ErrorResponse {
                    error: format!("get user error: {}", message),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                },
            Self::CreateTransactionError(message) =>
                ErrorResponse {
                    error: format!("create transaction error: {}", message),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                },
            Self::UnExpectedError(message) =>
                ErrorResponse {
                    error: format!("unexpected error: {}", message),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                },
        }
    }
}
