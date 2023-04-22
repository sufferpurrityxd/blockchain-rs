use serde::{
    Serialize,
    Deserialize,
};
use crate::tx::Tx;


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Block {
    pub index: u32,
    pub hash: String,
    pub prevblock_hash: String,
    pub txs: Vec<Tx>,
    pub nonce: u32,
    pub difficulty: usize,
    pub timestamp: i64,
}


impl Block {
    pub fn new(
        prevblock_index: u32,
        prevblock_hash: String,
        txs: Vec<Tx>,
        difficulty: usize,
    ) -> Self {
        let mut block = Self {
            index: prevblock_index+1,
            hash: String::new(),
            prevblock_hash: prevblock_hash,
            txs: txs,
            nonce: 0,
            difficulty: difficulty,
            timestamp: chrono::Utc::now().timestamp(),
        };
        let _ = block.proof_of_work();
        log::info!("Mined new block: {:?}", block);
        return block
    }

    pub fn prepare_hash(&self) ->  Result<String, Box<bincode::ErrorKind>> {
        return Ok(sha256::digest(bincode::serialize(&self)?.as_slice()));
    }

    pub fn proof_of_work(&mut self) -> std::io::Result<()> {
        let hash_starts_with = self.hash_starts_with();
        loop {
            let hash = self.prepare_hash().unwrap();
            if hash.starts_with(&hash_starts_with) {
                self.hash = hash;
                return Ok(());
            }
            self.nonce += 1;
        }
    }

    pub fn hash_starts_with(&self) -> String {
        let mut hash_starts_with = "".to_string();
        for _ in 0..self.difficulty {
            hash_starts_with = format!("{}0", hash_starts_with);
        }
        return hash_starts_with;
    }
}
