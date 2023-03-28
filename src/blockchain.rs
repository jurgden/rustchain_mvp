use crate::block::Block;
use crate::proof_of_work::ProofOfWork;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub proof_of_work: ProofOfWork,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let proof_of_work = ProofOfWork::new(difficulty);
        let genesis_block = proof_of_work.mine("0".to_string(), vec![]).1;
        Blockchain {
            blocks: vec![genesis_block],
            proof_of_work,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, nonce: u64) {
        let prev_block_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(prev_block_hash, transactions, nonce);
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self, difficulty: usize) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1];

            if !current_block.is_valid_proof_of_work(difficulty)
                || current_block.prev_block_hash != prev_block.hash
            {
                return false;
            }
        }
        true
    }
}
