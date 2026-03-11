

use crate::models::Message;
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
        self.repo.get_messages(limit,offset).await
    }
    pub async fn create_message(&self, data: CreateMessageData) -> Result<Message, AppError> {
        self.repo.create_message(data.content).await
    }
    pub async fn update_message(&self, id: i32, data: UpdateMessageData) -> Result<Message, AppError> {
        self.repo.update_message(id, data.content).await
    }
    pub async fn delete_message(&self, id: i32) -> Result<Message, AppError> {
        self.repo.delete_message(id).await
    }
}
