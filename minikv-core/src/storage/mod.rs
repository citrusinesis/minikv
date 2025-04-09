pub mod error;
pub mod mem;

pub use error::KvError;
pub use mem::{InMemoryStorage, SharedInMemoryStorage};

pub type KvResult<T> = Result<T, KvError>;

pub trait Storage {
    fn set(&self, key: String, value: String) -> KvResult<()>;
    fn get(&self, key: &str) -> KvResult<String>;
    fn delete(&self, key: &str) -> KvResult<()>;
}

pub trait SharedStorage: Storage + Send + Sync {}
impl<T> SharedStorage for T where T: Storage + Send + Sync {}
