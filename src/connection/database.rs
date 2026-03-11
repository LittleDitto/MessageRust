use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn create_pool() -> PgPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("Failed to create pool")
    }

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
