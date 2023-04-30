use serde::{Deserialize, Serialize};
use crate::network::actions::SendTransactionMeta;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
  pub from: String,
  pub to: String,
  pub amount: f32,
}

impl From<SendTransactionMeta> for Transaction {
  fn from(value: SendTransactionMeta) -> Self {
    return Self {
      from: value.from,
      to: value.to,
      amount: value.amount,
    };
  }
}
