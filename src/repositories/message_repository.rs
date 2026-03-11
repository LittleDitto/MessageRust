use sqlx::PgPool;
use tracing::error;

use crate::models::message::Message;
use crate::errors::app_error::AppError;

#[derive(Clone)]
pub struct MessageRepository{
    pool: PgPool,
}

impl MessageRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_messages(&self, limit: i64, offset: i64) -> Result<Vec<Message>, AppError> {
        let messages = sqlx::query_as::<_, Message>("SELECT id, content FROM messages ORDER BY id DESC LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                error!("Database error fetching messages: {:?}", err);
                AppError::DatabaseError(err.to_string())
            })?;

        Ok(messages)
    }

    pub async fn create_message(&self, content: String) -> Result<Message, AppError> {
        let message = sqlx::query_as::<_, Message>("INSERT INTO messages (content) VALUES ($1) RETURNING id, content")
            .bind(content)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                error!("Database error creating message: {:?}", err);
                AppError::DatabaseError(err.to_string())
            })?;

        Ok(message)
    }

    pub async fn update_message(&self, id: i32, content: String) -> Result<Message, AppError> {
        let message = sqlx::query_as::<_, Message>("UPDATE messages SET content = $1 WHERE id = $2 RETURNING id, content")
            .bind(content)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                error!("Database error updating message: {:?}", err);
                AppError::DatabaseError(err.to_string())
            })?;

        Ok(message)
    }

    pub async fn delete_message(&self, id: i32) -> Result<Message, AppError> {
        let message = sqlx::query_as::<_, Message>("DELETE FROM messages WHERE id = $1 RETURNING id, content")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                error!("Database error deleting message: {:?}", err);
                AppError::DatabaseError(err.to_string())
            })?;

        Ok(message)
    }
}
