pub mod storage;

use crate::storage::{KvResult, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "cmd", rename_all = "lowercase")]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
}

impl Command {
    pub fn execute<T: Storage + ?Sized>(&self, store: &T) -> KvResult<String> {
        match self {
            Command::Set { key, value } => {
                store.set(key.clone(), value.clone())?;
                Ok("OK".into())
            }
            Command::Get { key } => {
                let v = store.get(key)?;
                Ok(v)
            }
            Command::Del { key } => {
                store.delete(key)?;
                Ok("Deleted".into())
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Response {
    Ok { value: String },
    Error { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::InMemoryStorage;

    #[test]
    fn test_command_execution() {
        let store = InMemoryStorage::new();

        let cmd = Command::Set {
            key: "test".into(),
            value: "value".into(),
        };
        assert_eq!(cmd.execute(&store).unwrap(), "OK");

        let cmd = Command::Get { key: "test".into() };
        assert_eq!(cmd.execute(&store).unwrap(), "value");

        let cmd = Command::Del { key: "test".into() };
        assert_eq!(cmd.execute(&store).unwrap(), "Deleted");

        let cmd = Command::Get { key: "test".into() };
        assert!(cmd.execute(&store).is_err());
    }
}
