use std::time::{
    Duration,
    UNIX_EPOCH,
};
use chrono::{
    DateTime,
    Utc
};
use crate::{block::Block, address::Address, blockchain::Blockchain};
use crate::tx::Tx;

const MAX_TRANSACTIONS_IN_BLOCK: u32 = 5000;


pub struct Miner {
    pub address: Address,
    pub blockchain: Blockchain,
}


impl Miner {
    pub fn new(address: Address, blockchain: Blockchain) -> Self {
        return Self {
            address,
            blockchain,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            if let Some(b) = self.blockchain.blocks.clone().last() {
                if !self.is_ready_to_sign_txs(&b) { continue };
                let signed_txs = self.sign_txs();
                match signed_txs {
                    Some(txs) => {
                        // TODO: Add dynamic block difficulty (count of 0 in block_hash)
                        let difficulty = 1;
                        let block = Block::new(
                            b.index.clone(),
                            b.hash.clone(),
                            txs,
                            difficulty,
                        );
                        self.blockchain.add_block(block);
                        self.blockchain.unsigned_txs = vec![Tx::default()];
                        return Ok(());
                    }
                    None => log::error!("All transactions in blockchain queue is invalid"),
                }
            }
        }
    }

    fn is_ready_to_sign_txs(&self, b: &Block) -> bool {
        let unsigned_txs = self.blockchain.unsigned_txs.clone();
        let block_ts = DateTime::<Utc>::from(
            UNIX_EPOCH + Duration::from_secs(
                b
                    .timestamp
                    .clone() as u64,
            ),
        );
        let now_ts = Utc::now();
        // If last block created 10 minutes ago and we have more than one transactions,
        //
        // then we need to sign new transactions
        if now_ts > block_ts + chrono::Duration::from_std(Duration::from_secs(10 * 60)).unwrap() && unsigned_txs.len() > 0 {
            return true;
        }
        // If unsigned transactions > 5000 (max count of transactions in block)
        if unsigned_txs.len() >= MAX_TRANSACTIONS_IN_BLOCK as usize {
            return true;
        }

        return false;
    }

    fn sign_txs(&self) -> Option<Vec<Tx>> {
        let mut txs = vec![];
        for tx in self.blockchain.unsigned_txs.to_vec() {
            if tx.is_valid_tx() {
                txs.push(tx);
            }
        }
        return Some(txs);
    }

}