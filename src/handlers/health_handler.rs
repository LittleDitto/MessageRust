use axum::{extract::State, Json};
use serde_json::json;

use crate::state::AppState;
use crate::errors::AppError;

pub async fn health_handler(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query("SELECT 1")
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Database connection failed: {}", e);
            AppError::InternalServerError("Database connection failed".to_string())
        })?;
    Ok(Json(json!({
        "status": "ok",
        "message": "Service is healthy"
    })))
}

