mod tx;
mod block;
mod blockchain;
mod address;
mod miner;
mod network;

fn main() -> std::io::Result<()> {
    pretty_env_logger::try_init_timed().unwrap();

    return Ok(());
}
