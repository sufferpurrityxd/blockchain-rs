use serde::{Serialize, Deserialize};

use crate::chain::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
  pub index: usize,
  pub hash: String,
  pub prevblock_hash: String,
  pub timestamp: i64,
  pub transactions: Vec<Transaction>,
  pub nonce: usize,
}

impl Block {
  pub fn new(
    prevblock_hash: String,
    prevblock_index: usize,
    transactions: Vec<Transaction>,
    chain_difficulty: usize,
  ) -> Self {
    let mut block = Self {
      index: prevblock_index+1,
      hash: Default::default(),
      prevblock_hash: prevblock_hash,
      timestamp: Default::default(),
      transactions: transactions,
      nonce: 0,
    };
    block.proof_of_work(chain_difficulty);
    return block;
  }

  pub fn prepare_hash(
    &self,
  ) -> String {
    return sha256::digest(bincode::serialize(&self).unwrap().as_slice());
  }

  pub fn proof_of_work(
    &mut self,
    chain_difficulty: usize,
  ) {
    let mut block_prefix = "".to_string();
    for _ in 0..chain_difficulty { block_prefix = format!("{}0", block_prefix) }

    loop {
      let hash = self.prepare_hash();
      if hash.starts_with(&block_prefix) {
        self.hash = hash;
        self.timestamp = chrono::Utc::now().timestamp();
        break;
      }
      self.nonce+=1;
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::chain::block::Block;

  #[test]
  fn test_chain_difficulty() {
    let block1 = Block::new("".to_string(), 0, Default::default(), 2);
    let block2 = Block::new("".to_string(), 0, Default::default(), 3);
    let block3 = Block::new("".to_string(), 0, Default::default(), 4);
    assert!(block1.hash.starts_with("00"));
    assert!(block2.hash.starts_with("000"));
    assert!(block3.hash.starts_with("0000"));
  }
}
