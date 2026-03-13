use sqlx::PgPool;
use sqlx::Transaction;
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

    pub async fn begin_transaction(&self) -> Result<Transaction<'_, sqlx::Postgres>, AppError> {
        self.pool.begin().await.map_err(|err| {
            error!("Database error starting transaction: {:?}", err);
            AppError::DatabaseError(err.to_string())
        })
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

    pub async fn create_message(
        &self, 
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, 
        content: String
    ) -> Result<Message, AppError> {
        let message = sqlx::query_as::<_, Message>("INSERT INTO messages (content) VALUES ($1) RETURNING id, content")
            .bind(content)
            .fetch_one(tx.as_mut())
            .await
            .map_err(|err| {
                error!("Database error creating message: {:?}", err);
                AppError::DatabaseError(err.to_string())
            })?;

        Ok(message)
    }

    pub async fn update_message(
        &self, 
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, 
        id: i32, 
        content: String
    ) -> Result<Message, AppError> {
        let message = sqlx::query_as::<_, Message>("UPDATE messages SET content = $1 WHERE id = $2 RETURNING id, content")
            .bind(content)
            .bind(id)
            .fetch_one(tx.as_mut())
            .await
            .map_err(|err| {
                error!("Database error updating message: {:?}", err);
                AppError::DatabaseError(err.to_string())
            })?;

        Ok(message)
    }

    pub async fn delete_message(
        &self, 
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, 
        id: i32
    ) -> Result<Message, AppError> {
        let message = sqlx::query_as::<_, Message>("DELETE FROM messages WHERE id = $1 RETURNING id, content")
            .bind(id)
            .fetch_one(tx.as_mut())
            .await
            .map_err(|err| {
                error!("Database error deleting message: {:?}", err);
                AppError::DatabaseError(err.to_string())
            })?;

        Ok(message)
    }
}
