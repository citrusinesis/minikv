use futures_util::FutureExt;
use minikv_core::{Command, storage::SharedStorage};
use std::sync::Arc;

use crate::service::wrapper::ServiceWrapper;

#[derive(Clone)]
pub struct CommandService {
    store: Arc<dyn SharedStorage>,
}

impl CommandService {
    pub fn new(store: Arc<dyn SharedStorage>) -> Self {
        Self { store }
    }

    pub fn wrap(&self) -> ServiceWrapper {
        let store = self.store.clone();
        ServiceWrapper::new(Arc::new(move |cmd: Command| {
            let store = store.clone();
            async move { cmd.execute(store.as_ref()) }.boxed()
        }))
    }
}
