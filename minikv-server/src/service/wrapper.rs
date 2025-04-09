use minikv_core::{Command, storage::KvError};

use futures_util::future::BoxFuture;
use std::{
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;

#[derive(Clone)]
pub struct ServiceWrapper {
    pub handler: Arc<dyn Fn(Command) -> BoxFuture<'static, Result<String, KvError>> + Send + Sync>,
}

impl ServiceWrapper {
    pub fn new(
        handler: Arc<dyn Fn(Command) -> BoxFuture<'static, Result<String, KvError>> + Send + Sync>,
    ) -> Self {
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
