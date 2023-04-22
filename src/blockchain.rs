use std::sync::{Arc, Mutex};
use crate::{
    tx::Tx,
    block::Block,
};

pub struct Blockchain {
    pub blocks: Arc<Mutex<Vec<Block>>>,

    // Transactions that will be included in the next block after they are validated
    pub unsigned_txs: Arc<Mutex<Vec<Tx>>>,
}

impl Blockchain {
    pub fn new(
        blocks: Option<Vec<Block>>,
        unsigned_txs: Option<Vec<Tx>>,

    ) -> Self {
        return Self { 
            blocks: match blocks {
                // If that a first run of blockchain
                None => Arc::new(Mutex::new(vec![create_genesis_block()])),

                // If we are accept blocks from chain,
                // so we are dont need to create are vec with blocks
                Some(blocks) => Arc::new(Mutex::new(blocks)),
            },
            unsigned_txs: match unsigned_txs {
                // If that a first run of blockchain
                None => Arc::new(Mutex::new(vec![Default::default()])),
                // If we are accept blocks from chain,
                // so we are dont need to create are vec with unsigned txs
                Some(unsigned_txs) => Arc::new(Mutex::new(unsigned_txs)),
            }
        }
    }



    pub fn add_unsigned_tx(&mut self, tx: Tx) {
        log::debug!("Added new unsigned transaction: {tx:?}");
        self.unsigned_txs.lock().unwrap().push(tx)
    }
}


fn create_genesis_block() -> Block {
    return Block {
        index: 0,
        hash: "".to_string(),
        prevblock_hash: "".to_string(),
        txs: Vec::default(),
        nonce: 0,
        timestamp: chrono::Utc::now().timestamp(),
    };
}