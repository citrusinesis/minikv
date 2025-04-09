use minikv_core::Command;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service};
use tracing::{info, warn};

#[derive(Clone)]
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> Service<Command> for LoggingMiddleware<S>
where
    S: Service<Command, Response = String, Error = minikv_core::storage::KvError>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Command) -> Self::Future {
        let start = Instant::now();
        let mut inner = self.inner.clone();
        Box::pin(async move {
            let result = inner.call(req.clone()).await;
            match &result {
                Ok(val) => info!(?req, %val, took = ?start.elapsed(), "Handled command"),
                Err(e) => warn!(?req, ?e, took = ?start.elapsed(), "Command failed"),
            }
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::future::{Ready, ready};
    use std::task::{Context, Poll};

    #[derive(Clone)]
    struct MockService;

    impl Service<Command> for MockService {
        type Response = String;
        type Error = minikv_core::storage::KvError;
        type Future = Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _: Command) -> Self::Future {
            ready(Ok("test response".into()))
        }
    }

    #[tokio::test]
    async fn test_logging_middleware() {
        let service = LoggingLayer.layer(MockService);
        let mut service = service;

        let cmd = Command::Get { key: "test".into() };

        let response = service.call(cmd).await.unwrap();
        assert_eq!(response, "test response");
    }
}
