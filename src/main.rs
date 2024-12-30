use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Block {
    data: String,
    hash: String,
    prev_hash: Option<String>,
    timestamp: u64,
}

impl Block {
    fn new(data: String, prev_hash: Option<String>) -> Block {
        let timestamp = Block::get_current_timestamp();
        let hash = Block::calculate_hash(&data, &prev_hash, timestamp);
        Block {
            data,
            hash,
            prev_hash,
            timestamp,
        }
    }

    fn calculate_hash(data: &str, prev_hash: &Option<String>, timestamp: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        if let Some(prev_hash) = prev_hash {
            hasher.update(prev_hash);
        }
        hasher.update(timestamp.to_string());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn get_current_timestamp() -> u64 {
        let start = SystemTime::now();
        start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { blocks: Vec::new() }
    }

    fn add_block(&mut self, data: String) {
        let prev_hash = self.blocks.last().map(|block| block.hash.clone());
        let new_block = Block::new(data, prev_hash);
        self.blocks.push(new_block);
    }
    fn print_chain(&self) {
        for block in &self.blocks {
            println!("{:?}", block);
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("First Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Third Block".to_string());

    blockchain.print_chain();
}
