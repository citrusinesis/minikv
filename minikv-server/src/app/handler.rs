use minikv_core::{
    Command,
    storage::{KvError, SharedStorage},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct CommandExecutor {
    store: Arc<dyn SharedStorage>,
}

impl CommandExecutor {
    pub fn new(store: Arc<dyn SharedStorage>) -> Self {
        Self { store }
    }

    pub async fn handle(&self, cmd: Command) -> Result<String, KvError> {
        cmd.execute(self.store.as_ref())
    }
}
