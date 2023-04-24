mod tx;
mod block;
mod blockchain;
mod address;
mod miner;
mod network;
mod storage;
mod node;
mod user;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::try_init_timed().unwrap();

    return Ok(());
}
