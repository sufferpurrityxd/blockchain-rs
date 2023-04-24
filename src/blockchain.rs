use crate::{
    tx::Tx,
    block::Block,
};


pub struct Blockchain {
    pub blocks: Vec<Block>,

    // Transactions that will be included in the next block after they are validated
    pub unsigned_txs: Vec<Tx>,

    // Difficulty for block hash
    //
    // Examples hashes:
    //      000000000000000000054f98cb7d450451ae8926a8da78c74b7658d49565ba37
    //      difficulty=19
    //
    //      00000000004124f1d54f98cb7d450451ae8926a8da78c74b7658d49565ba37f1
    //      difficulty=10
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(
        blocks: Option<Vec<Block>>,
        unsigned_txs: Option<Vec<Tx>>,
        difficulty: Option<usize>,
    ) -> Self {
        return Self {
            blocks: match blocks {
                // If that a first run of blockchain
                None => vec![create_genesis_block()],

                // If we are accept blocks from chain,
                // so we are dont need to create are vec with blocks
                Some(blocks) => blocks,
            },
            unsigned_txs: match unsigned_txs {
                // If that a first run of blockchain
                None => vec![Default::default()],
                // If we are accept blocks from chain,
                // so we are dont need to create are vec with unsigned txs
                Some(unsigned_txs) => unsigned_txs,
            },
            difficulty: match difficulty {
                None => 4,
                Some(difficulty) => difficulty,
            },
        }
    }

    pub fn add_block(&mut self, block: Block) {
        log::debug!("Added new block: {block:?}");
        self.blocks.push(block);
    }

    pub fn add_unsigned_tx(&mut self, tx: Tx) {
        log::debug!("Added new unsigned transaction: {tx:?}");
        self.unsigned_txs.push(tx)
    }
}


fn create_genesis_block() -> Block {
    return Block {
        index: 0,
        hash: "".to_string(),
        prevblock_hash: "".to_string(),
        txs: Vec::default(),
        nonce: 0,
        difficulty: 0,
        timestamp: chrono::Utc::now().timestamp(),
    };
}
