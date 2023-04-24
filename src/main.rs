use futures::FutureExt;
use crate::network::run_node;
use crate::storage::Storage;

mod tx;
mod block;
mod blockchain;
mod address;
mod miner;
mod network;
mod storage;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::try_init_timed().unwrap();
    let storage = Storage::new();
    let _ = run_node().await;

    return Ok(());
}
