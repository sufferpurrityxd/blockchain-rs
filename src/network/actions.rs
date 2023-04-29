use serde::{Deserialize, Serialize};
use crate::chain::{block::Block};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockSyncMeta {
  pub key: i32,
  pub block: Block,
}


#[derive(Serialize, Deserialize)]
pub enum Command {
  // Sync block around miner -> local node
  AddBlock(BlockSyncMeta),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
  // Sync block around global node -> miner
  SyncBlock(BlockSyncMeta)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GossipsubEvent {
  // Sync block around global nodes <-> local node
  SyncNetworkBlock(BlockSyncMeta)
}
