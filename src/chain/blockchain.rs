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
  pub fn new(blocks: Option<Vec<Block>>) -> Self {
    return Self {
      blocks: match blocks {
        Some(blocks) => blocks,
        None => vec![create_genesis_block()],
      },
      transactions: Default::default(),
      difficulty: 4
    }
  }

  pub fn add_block(&mut self, block: Block) { self.blocks.push(block) }

  pub fn add_transaction(&mut self, transaction: Transaction) { self.transactions.push(transaction) }

  pub fn address_balance(&self, _address: String) -> f32 {
    let mut b = 0.0;
    for block in self.blocks.iter() {
      for transaction in block.transactions.iter() {
        if transaction.to == _address {
          b += transaction.amount;
        } else if transaction.from == _address {
          b -= transaction.amount;
        }
      }
    }
    return b;
  }


  pub fn is_valid_transaction(&self, _transaction: &Transaction) -> bool {
    return if self.address_balance(_transaction.from.clone()) >= _transaction.amount {
      true
    } else {
      false
    }
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
