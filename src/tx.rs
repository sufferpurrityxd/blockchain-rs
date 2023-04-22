use serde::{
    Serialize,
    Deserialize,
};
use crate::address::Address;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tx {
    pub txid: String,
    pub addr_from: Address,
    pub addr_to: Address,
    pub amount: f32,
}


impl Tx {
    pub fn new(addr_from: Address, addr_to: Address, amount: f32) -> Self {
        let mut tx = Self {
            txid: Default::default(),
            addr_from: addr_from,
            addr_to: addr_to,
            amount: amount,
        };
        tx.txid = tx.prepare_hash().unwrap();
        return tx;
    }

    pub fn prepare_hash(&mut self) ->  Result<String, Box<bincode::ErrorKind>> {
        return Ok(sha256::digest(bincode::serialize(&self)?.as_slice()));
    }

    pub fn is_valid_tx(&self) -> bool {
        // TODO
       return true;
    }
}
