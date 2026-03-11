use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;

mod state;
mod data;
mod connection;
mod repositories;
mod handlers;
mod models;
mod routes;
mod bloc;
mod errors;

use crate::routes::routes;
use crate::bloc::MessageBloc;
use crate::repositories::MessageRepository;
use crate::connection::create_pool;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "microservices_app=debug,tower=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    tracing::info!("Starting server...");

    let pool = create_pool().await;
    let message_repository = MessageRepository::new(pool);
    let message_bloc = MessageBloc::new(message_repository);
    let app_state = AppState {
        message_bloc,
    };

    let app = routes().layer(TraceLayer::new_for_http()).with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

