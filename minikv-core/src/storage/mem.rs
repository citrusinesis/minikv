use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
};

use super::{KvError, KvResult, Storage};

mod inner {
    use super::KvError;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Storage {
        store: HashMap<String, String>,
    }

    impl Storage {
        pub fn set(&mut self, key: String, value: String) {
            self.store.insert(key, value);
        }

        pub fn get(&self, key: &str) -> Result<String, KvError> {
            self.store.get(key).cloned().ok_or(KvError::KeyNotFound)
        }

        pub fn delete(&mut self, key: &str) -> Result<(), KvError> {
            if self.store.remove(key).is_some() {
                Ok(())
            } else {
                Err(KvError::KeyNotFound)
            }
        }
    }
}

pub struct InMemoryStorage {
    inner: RefCell<inner::Storage>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(inner::Storage::default()),
        }
    }
}

impl Storage for InMemoryStorage {
    fn set(&self, key: String, value: String) -> KvResult<()> {
        self.inner.borrow_mut().set(key, value);
        Ok(())
    }

    fn get(&self, key: &str) -> KvResult<String> {
        self.inner.borrow().get(key)
    }

    fn delete(&self, key: &str) -> KvResult<()> {
        self.inner.borrow_mut().delete(key)
    }
}

#[derive(Clone)]
pub struct SharedInMemoryStorage {
    inner: Arc<RwLock<inner::Storage>>,
}

impl SharedInMemoryStorage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner::Storage::default())),
        }
    }
}

impl Storage for SharedInMemoryStorage {
    fn set(&self, key: String, value: String) -> KvResult<()> {
        self.inner
            .write()
            .map_err(|e| KvError::Internal(e.to_string()))?
            .set(key, value);
        Ok(())
    }

    fn get(&self, key: &str) -> KvResult<String> {
        self.inner
            .read()
            .map_err(|e| KvError::Internal(e.to_string()))?
            .get(key)
    }

    fn delete(&self, key: &str) -> KvResult<()> {
        self.inner
            .write()
            .map_err(|e| KvError::Internal(e.to_string()))?
            .delete(key)
    }
}
