use clap::Parser;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone)]
pub enum StorageKind {
    InMemory,
    // File(String),
}

#[derive(Parser, Debug)]
#[command(
    name = "MiniKV Server",
    version,
    about = "A tiny key-value store in Rust."
)]
pub struct CliArgs {
    #[arg(long)]
    pub addr: Option<String>,

    #[arg(long)]
    pub storage: Option<String>,

    #[arg(long)]
    pub config: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub addr: Option<String>,
    pub storage: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub addr: String,
    pub storage: StorageKind,
}

impl Config {
    pub fn load() -> Self {
        let args = CliArgs::parse();

        let mut file_config = FileConfig {
            addr: None,
            storage: None,
        };

        if let Some(path) = args.config.as_ref() {
            let content = fs::read_to_string(path).expect("Failed to read config file");
            file_config =
                toml::from_str(&content).expect("Failed to parse config file (TOML format)");
        }

        let addr = args
            .addr
            .or(file_config.addr)
            .unwrap_or_else(|| "127.0.0.1:4000".to_string());

        let storage = args
            .storage
            .or(file_config.storage)
            .unwrap_or_else(|| "inmemory".to_string());

        let storage_kind = match storage.as_str() {
            "inmemory" => StorageKind::InMemory,
            // "file" => StorageKind::File("data.db".to_string()),
            other => {
                eprintln!("Unsupported storage backend: {}", other);
                std::process::exit(1);
            }
        };

        Config {
            addr,
            storage: storage_kind,
        }
    }
}
