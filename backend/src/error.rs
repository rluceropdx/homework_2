use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;

pub enum AppError {
    Question(QuestionError),
    Any(anyhow::Error)
}

#[derive(derive_more::Display)]
pub enum QuestionError {
    InvalidId,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Question(err) => match err {
                QuestionError::InvalidId => (StatusCode::NOT_FOUND, err.to_string()),
            },
            AppError::Any(err) => {
                let message = format!("Internal server error! {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        };

        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}
