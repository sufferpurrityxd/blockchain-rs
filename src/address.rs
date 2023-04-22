use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Address (pub String);
