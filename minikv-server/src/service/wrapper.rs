use minikv_core::{Command, storage::KvError};

use futures_util::future::BoxFuture;
use std::{
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;

pub type Handler =
    Arc<dyn Fn(Command) -> BoxFuture<'static, Result<String, KvError>> + Send + Sync>;

#[derive(Clone)]
pub struct ServiceWrapper {
    pub handler: Handler,
}

impl ServiceWrapper {
    pub fn new(handler: Handler) -> Self {
        Self { handler }
    }
}

impl Service<Command> for ServiceWrapper {
    type Response = String;
    type Error = KvError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Command) -> Self::Future {
        (self.handler)(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::Service;

    #[tokio::test]
    async fn test_service_wrapper() {
        let handler: Handler = Arc::new(|cmd| {
            Box::pin(async move {
                match cmd {
                    Command::Get { key } => Ok(format!("value for {}", key)),
                    _ => Ok("OK".into()),
                }
            })
        });

        let mut service = ServiceWrapper::new(handler);

        assert!(
            service
                .poll_ready(&mut std::task::Context::from_waker(
                    futures_util::task::noop_waker_ref()
                ))
                .is_ready()
        );

        let response = service
            .call(Command::Get { key: "test".into() })
            .await
            .unwrap();
        assert_eq!(response, "value for test");
    }
}
