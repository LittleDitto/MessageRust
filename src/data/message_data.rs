use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize,Validate)]
pub struct CreateMessageData {
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}

#[derive(Debug, Deserialize,Validate)]
pub struct UpdateMessageData {
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}

#[derive(Debug, Serialize, Validate)]
pub struct MessageResponseData {
    pub id: i32,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}
