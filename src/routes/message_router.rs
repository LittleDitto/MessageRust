use axum::{Router, routing::{get, post, put, delete}};

use crate::state::AppState;
use crate::handlers::MessageHandler;    

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/messages", get(MessageHandler::get_messages))
        .route("/messages", post(MessageHandler::create_message))
        .route("/messages/{id}", put(MessageHandler::update_message))
        .route("/messages/{id}", delete(MessageHandler::delete_message))
}
