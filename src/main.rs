mod models;
mod storage;

use storage::Storage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut blockchain = Storage::load_blockchain()?;
}
