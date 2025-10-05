mod models;
mod storage;
mod network;
mod cli;

use storage::Storage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut blockchain = Storage::load_blockchain()?;
}
