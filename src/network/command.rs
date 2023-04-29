use serde::{Deserialize, Serialize};
use crate::chain::{
  block::Block,
  transaction::Transaction,
};

#[derive(Serialize, Deserialize)]
pub enum Command {
  AddBlock {
    key: i32,
    block: Block,
  },
  Transaction(Transaction),
}
