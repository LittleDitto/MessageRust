
use sqlx::PgPool;

use crate::bloc::MessageBloc;

#[derive(Clone)]
pub struct AppState {
    pub message_bloc: MessageBloc,
    pub db_pool: PgPool,
}
