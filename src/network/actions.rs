use serde::{Deserialize, Serialize};
use crate::chain::{block::Block};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockSyncMeta {
  pub key: i32,
  pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendTransactionMeta {
  pub from: String,
  pub to: String,
  pub amount: f32,
}

#[derive(Serialize, Deserialize)]
pub enum Command {
  // Sync block around miner -> local node
  AddBlock(BlockSyncMeta),
  // Transactions
  SendTransaction(SendTransactionMeta),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
  // Sync block around global node -> miner
  SyncBlock(BlockSyncMeta),
  SendTransaction(SendTransactionMeta),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GossipsubEvent {
  // Sync block around global nodes <-> local node
  SyncNetworkBlock(BlockSyncMeta),
  // Transactions
  SendNetworkTransaction(SendTransactionMeta),
}
