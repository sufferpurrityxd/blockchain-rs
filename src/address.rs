use serde::{Deserialize, Serialize};


// TODO
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Address (pub String);
