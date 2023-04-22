mod tx;
mod block;
mod blockchain;
mod address;
mod miner;

fn main() -> std::io::Result<()> {
    pretty_env_logger::try_init_timed().unwrap();

    return Ok(());
}
