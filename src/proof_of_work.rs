use crate::block::Block;

pub struct ProofOfWork {
    pub difficulty: usize,
}

impl ProofOfWork {
    pub fn new(difficulty: usize) -> Self {
        ProofOfWork { difficulty }
    }

    pub fn mine(&self, prev_block_hash: Strgin, transactions: vec<Transaction>) -> (u64, Block) {
        let mut nonce = 0;
        loop {
            let block = Block::new(prev_block_hash.clone(), transactions.clone(), nonce);
            if block.is_valid_proof_of_work(self.difficulty) {
                return (nonce, block);
            }
            nonce += 1;
        }
    }
}
