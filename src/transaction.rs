use serde::{Serialize, Deserialize};
use rsa::RsaPublicKey;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub addr_from: String,
    pub addr_to: String,
    pub count: f32,
}
