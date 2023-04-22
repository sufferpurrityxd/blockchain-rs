use serde::{
    Serialize,
    Deserialize,
};
use crate::tx::Tx;


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Block {
    pub index: u32,
    pub hash: String,
    pub prevblock_hash: String,
    pub txs: Vec<Tx>,
    pub nonce: u32,
    pub timestamp: i64,
}


impl Block {
    pub fn new(prevblock_index: u32, prevblock_hash: String, txs: Vec<Tx>) -> Self {
        let mut block = Self {
            index: prevblock_index+1,
            hash: String::new(),
            prevblock_hash: prevblock_hash,
            nonce: 0,
            txs: txs,
            timestamp: chrono::Utc::now().timestamp(),
        };
        return block
    }

    pub fn prepare_hash(&self) ->  Result<String, Box<bincode::ErrorKind>> {
        return Ok(sha256::digest(bincode::serialize(&self)?.as_slice()));
    }

}
