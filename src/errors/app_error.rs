use axum::{http::StatusCode, response::{IntoResponse,Response}, Json};
use serde_json::json;
use tracing::{error, warn, info};

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String), 
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    Unauthorized(String),
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DatabaseError(err) => {
                // Log the actual error internally with tracing
                error!("Database error occurred: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::NotFound(msg) => {
                warn!("Resource not found: {}", msg);
                (StatusCode::NOT_FOUND, msg)
            }
            AppError::InternalServerError(msg) => {
                error!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            AppError::BadRequest(msg) => {
                info!("Bad request: {}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::Unauthorized(msg) => {
                warn!("Unauthorized access: {}", msg);
                (StatusCode::UNAUTHORIZED, msg)
            }
            AppError::Conflict(msg) => {
                info!("Conflict: {}", msg);
                (StatusCode::CONFLICT, msg)
            }
        };
        let body = Json(json!({ 
            "error": message 
            }
        )
        );
        (status, body).into_response()

    }
}
