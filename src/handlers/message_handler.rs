use axum::{extract::{State, Json, Query, Path}};
use validator::Validate;

use crate::{data::MessageResponseData, models::Message};
use crate::state::AppState;
use crate::data::CreateMessageData;
use crate::data::UpdateMessageData;
use crate::errors::AppError;
use crate::data::Pagination;

pub struct MessageHandler;

impl MessageHandler {
    pub async fn get_messages(State(state): State<AppState>, Query(_param): Query<Pagination>) -> Result<Json<Vec<Message>>, AppError> {
        let _page = _param.page.unwrap_or(1);
        let _limit = _param.limit.unwrap_or(10);
        let _offset = (_page - 1) * _limit;

        let messages = state.message_bloc
            .get_messages(_limit, _offset)
            .await?;
        Ok(Json(messages))
    }

    pub async fn create_message(State(state): State<AppState>, Json(payload): Json<CreateMessageData>) -> Result<Json<MessageResponseData>, AppError> {
        payload.validate().map_err(|err| AppError::BadRequest(format!("Validation error: {}", err)))?;
        let message = state.message_bloc
            .create_message(payload)
            .await?;
        Ok(Json(MessageResponseData { 
            id: message.id, 
            content: message.content,
            }
        )
    )
    }

    pub async fn update_message(State(state): State<AppState>, Path(id): Path<i32>, Json(payload): Json<UpdateMessageData>) -> Result<Json<MessageResponseData>, AppError> {
        payload.validate().map_err(|err| AppError::BadRequest(format!("Validation error: {}", err)))?;
        let message = state.message_bloc
            .update_message(id, payload)
            .await?;
        Ok(Json(MessageResponseData { 
            id: message.id, 
            content: message.content,
            }
        )
    )
    }

    pub async fn delete_message(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<MessageResponseData>, AppError> {
        let message = state.message_bloc
            .delete_message(id)
            .await?;
        Ok(Json(MessageResponseData { 
            id: message.id, 
            content: message.content,
            }
        )
    )
    }
}
