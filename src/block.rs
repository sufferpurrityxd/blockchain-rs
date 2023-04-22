use serde::{Serialize, Deserialize};
use crate::errors::BlockError;
use crate::transaction::Transaction;

pub const HASH_DIFFICULTY: &str = "00";

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub hash: String,
    pub prevblock_hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u32,
    pub timestamp: i64,
}


impl Block {
    pub fn new(
        prevblock_index: u32,
        prevblock_hash: String,
        transactions: Vec<Transaction>
    ) -> Result<Self, BlockError> {
        let mut block = Self {
            index: prevblock_index+1,
            hash: String::new(),
            prevblock_hash: prevblock_hash,
            nonce: 0,
            transactions: transactions,
            timestamp: chrono::Utc::now().timestamp(),
        };
        block.start_proof_of_work()?;
        return Ok(block);
    }

    pub fn start_proof_of_work(&mut self) -> Result<(), BlockError> {
        loop {
            let hash = self.prepare_hash()?;
            if hash.starts_with(HASH_DIFFICULTY) {
                self.hash = hash;
                log::debug!("Mined new block with hash: {}, nocne: {}", self.hash, self.nonce);
                return Ok(());
            }
            self.nonce += 1
        }
    }


    pub fn prepare_hash(&self) ->  Result<String, BlockError> {
        return Ok(sha256::digest(bincode::serialize(&self)?.as_slice()));
    }
}
