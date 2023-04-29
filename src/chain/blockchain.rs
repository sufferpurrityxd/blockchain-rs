use crate::{
  chain::block::Block,
  chain::transaction::Transaction,
};

pub struct Blockchain {
  pub blocks: Vec<Block>,
  pub transactions: Vec<Transaction>,
  pub difficulty: usize,
}

impl Blockchain {
  pub fn new(
    blocks: Option<Vec<Block>>,
    transactions: Option<Vec<Transaction>>,
    difficulty: Option<usize>,
  ) -> Self {
    return Self {
      blocks: match blocks {
        Some(blocks) => blocks,
        None => vec![create_genesis_block()],
      },
      transactions: match transactions {
        None => Default::default(),
        Some(transactions) => transactions,
      },
      difficulty: match difficulty {
        None => 4,
        Some(difficulty) => difficulty,
      },
    }
  }

  pub fn add_block(&mut self, block: Block) { self.blocks.push(block) }

  pub fn is_valid_transaction(&self, _transaction: &Transaction) -> bool {
    // TODO
    return true;
  }

}

fn create_genesis_block() -> Block {
  return Block {
    index: 0,
    hash: "000000000000000000GENESIS".to_string(),
    prevblock_hash: "".to_string(),
    timestamp: chrono::Utc::now().timestamp(),
    transactions: vec![],
    nonce: 0,
  }
}
