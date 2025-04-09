mod app;
mod config;
mod server;
mod service;

use app::handler::CommandExecutor;
use config::{Config, StorageKind};
use service::{logging::LoggingLayer, wrapper::ServiceWrapper};

use futures_util::{FutureExt, future::BoxFuture};
use minikv_core::{
    Command,
    storage::{KvError, SharedInMemoryStorage, SharedStorage},
};
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

    let executor = Arc::new(CommandExecutor::new(store));
    let handler: Arc<dyn Fn(Command) -> BoxFuture<'static, Result<String, KvError>> + Send + Sync> = {
        let exec = executor.clone();
        Arc::new(move |cmd: Command| {
            let exec = exec.clone();
            async move { exec.handle(cmd).await }.boxed()
        })
    };

    let service = ServiceBuilder::new()
        .layer(LoggingLayer)
        .service(ServiceWrapper::new(handler));

    server::start_server(&config.addr, service).await;
}
