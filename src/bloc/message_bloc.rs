use crate::models::message::Message;
use crate::repositories::MessageRepository;
use crate::data::CreateMessageData;
use crate::data::UpdateMessageData;
use crate::errors::AppError;

#[derive(Clone)]
pub struct MessageBloc{
    repo: MessageRepository,
}

impl MessageBloc {
    pub fn new(repo: MessageRepository) -> Self {
        Self { repo }
    }
    
    pub async fn get_messages(&self, limit: i64, offset: i64) -> Result<Vec<Message>, AppError> {
        self.repo.get_messages(limit, offset).await
    }
    
    pub async fn create_message(&self, data: CreateMessageData) -> Result<Message, AppError> {
        let mut tx = self.repo.begin_transaction().await?;
        let bloc_message = self.repo.create_message(&mut tx, data.content).await?;
        tx.commit().await.map_err(|_e| AppError::DatabaseError("Commit failed".to_string()))?;
        Ok(bloc_message)
    }
    
    pub async fn update_message(&self, id: i32, data: UpdateMessageData) -> Result<Message, AppError> {
        let mut tx = self.repo.begin_transaction().await?;
        let bloc_message = self.repo.update_message(&mut tx, id, data.content).await?;
        tx.commit().await.map_err(|_e| AppError::DatabaseError("Commit failed".to_string()))?;
        Ok(bloc_message)
    }
    
    pub async fn delete_message(&self, id: i32) -> Result<Message, AppError> {
        let mut tx = self.repo.begin_transaction().await?;
        let bloc_message = self.repo.delete_message(&mut tx, id).await?;
        tx.commit().await.map_err(|_e| AppError::DatabaseError("Commit failed".to_string()))?;
        Ok(bloc_message)
    }
}
