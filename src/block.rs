use crate::transaction::Transaction;
use chrono::prelude::*;
use crypto_hash::{hex_digest, Algorithm};
use uuid::Uuid;

pub struct Block {
    pub prev_block_hash: String,
    pub timestamp: u64,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(prev_block_hash: String, transactions: Vec<Transaction>, nonce: u64) -> Self {
        let timestamp = Utc::now().timestamp() as u64;
        let mut block = Block {
            prev_block_hash,
            timestamp,
            nonce,
            transactions,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}",
            self.prev_block_hash,
            self.timestamp,
            self.nonce,
            self.transaction_hashes()
        );
        hex_digest(Algorithm::SHA256, input.as_bytes())
    }

    fn transaction_hashes(&self) -> String {
        self.transactions
            .iter()
            .map(|tx| tx.id.clone())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn is_valid_proof_of_work(&self, difficulty: usize) -> bool {
        let target = format!("{:0>width$}", "", width = difficulty);
        &self.hash[..difficulty] == target
    }
}
