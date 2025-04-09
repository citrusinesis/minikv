mod app;
mod config;
mod server;
mod service;

use app::handler::CommandService;
use config::{Config, StorageKind};
use service::logging::LoggingLayer;

use minikv_core::storage::{SharedInMemoryStorage, SharedStorage};
use std::sync::Arc;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_ansi(true)
        .with_target(false)
        .init();

    let config = Config::load();

    tracing::info!("Starting MiniKV server on {}", config.addr);

    let store: Arc<dyn SharedStorage> = match config.storage {
        StorageKind::InMemory => Arc::new(SharedInMemoryStorage::new()),
    };

    let service = ServiceBuilder::new()
        .layer(LoggingLayer)
        .service(CommandService::new(store).wrap());

    server::start_server(&config.addr, service).await;
}
