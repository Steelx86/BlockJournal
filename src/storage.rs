use crate::models::Blockchain;
use serde_json;
use std::fs;
use std::path::Path;

pub struct Storage {
    pub file_path: String,
}

impl Storage {
    pub fn new(file_path: String) -> Self {
        Storage { file_path }
    }

    pub fn save_blockchain(&self, blockchain: &Blockchain) -> std::io::Result<()> {
        let data = serde_json::to_string(blockchain).unwrap();
        fs::write(&self.file_path, data)
    }

    pub fn load_blockchain(&self) -> Option<Blockchain> {
        if Path::new(&self.file_path).exists() {
            let data = fs::read_to_string(&self.file_path).ok()?;
            let blockchain: Blockchain = serde_json::from_str(&data).ok()?;
            Some(blockchain)
        } else {
            None
        }
    }

    
}
