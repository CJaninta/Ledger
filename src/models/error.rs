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
}

impl IntoErrorResponse for APIError {
    fn error(&self) -> ErrorResponse {
        match self {
            Self::UserAlreadyExists(message) =>
                ErrorResponse {
                    error: format!("user is already exists: {}", message),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                },
            Self::CreateUserError(message) =>
                ErrorResponse {
                    error: format!("create user error: {}", message),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                },
        }
    }
}
