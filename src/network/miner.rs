use chrono::{
  DateTime,
  NaiveDateTime,
  Utc,
};
use futures::{SinkExt, channel::mpsc::{
  Sender,
  Receiver,
}};
use crate::{
  chain::{
    block::Block,
    blockchain::Blockchain,
  },
  network::actions::{
    BlockSyncMeta,
    Command,
    Event,
  },
};
use crate::chain::transaction::Transaction;


const MAX_TRANSACTIONS_IN_BLOCK: usize = 5000;

pub struct Miner {
  pub blockchain: Blockchain,
  pub command_tx: Sender<Command>,
  pub event_rx: Receiver<Event>,
}

impl Miner {
  pub fn new(
    blockchain: Blockchain,
    command_tx: Sender<Command>,
    event_rx: Receiver<Event>,
  ) -> Self {
    return Self {
      blockchain,
      command_tx,
      event_rx,
    }
  }

  pub async fn execute(&mut self)  {
    loop {
      if self.is_ready_to_sign() { self.process_new_block().await };
      if let Ok(Some(event)) = self.event_rx.try_next() { self.handle_event(event).await };
    }
  }

  async fn handle_event(&mut self, event: Event) {
    match event {
      Event::SyncBlock(meta)  => self.blockchain.add_block(meta.block),
      Event::SendTransaction(meta) => self.blockchain.add_transaction(Transaction::from(meta)),
    }
  }

  fn is_ready_to_sign(&self) -> bool {
    if self.blockchain.transactions.len() >= MAX_TRANSACTIONS_IN_BLOCK { return true }
    let last_block_dt = DateTime::<Utc>::from_utc(
      NaiveDateTime::from_timestamp_millis(
        self.blockchain.blocks.last().unwrap().timestamp)
        .unwrap(),
      Utc
    );
    if last_block_dt + chrono::Duration::minutes(1) > Utc::now() { return true };
    return false;
  }

  async fn process_new_block(&mut self) {
    // TODO: self.blockchain.transactions_iter()
    let mut valid_transactions = Vec::new();
    for transaction in self.blockchain.transactions.clone() {
      if self.blockchain.is_valid_transaction(&transaction) { valid_transactions.push(transaction) }
    }
    match self.blockchain.blocks.last() {
      Some(block) => {
        let block = Block::new(
          block.hash.clone(),
          block.index.clone(),
          valid_transactions,
          self.blockchain.difficulty.clone());
        match self.command_tx.send( Command::AddBlock(BlockSyncMeta {
          key: block.index.clone() as i32,
          block: block.clone(),
        })).await {
          Ok(_) => self.blockchain.add_block(block),
          Err(e) => log::error!("Failed to push new block into chain: {e:?}"),
        }
        self.blockchain.transactions = Vec::new();
      },
      None => log::error!("Got empty blockchain while creating new block")
    }

  }
}
