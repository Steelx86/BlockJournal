use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub timestamp: DateTime<Utc>,
    pub location: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub entry: JournalEntry,
    pub previous_hash: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}


impl JournalEntry {
    pub fn new(content: String, location: String) -> Self {
        let timestamp = Utc::now();
        JournalEntry {
            timestamp,
            location,
            content,
        }
    }

    pub fn hash(&self) -> String {
        let serialized = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }
}

impl Block {
    pub fn new(index: u64, entry: JournalEntry, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let mut block = Block {
            index,
            timestamp,
            entry,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let serialized = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_entry = JournalEntry::new(
            "Genesis Block".to_string(),
            "N/A".to_string(),
        );

        let genesis_block = Block::new(0, genesis_entry, "0".to_string());

        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_entry(&mut self, content: String, location: String) -> &Block {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let entry = JournalEntry::new(content, location);

        let index = self.chain.len() as u64;
        let new_block = Block::new(index, entry, previous_hash);

        self.chain.push(new_block);
        self.chain.last().unwrap()
    }
}
