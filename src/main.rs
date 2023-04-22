use crate::block::Block;
use crate::transaction::Transaction;

mod transaction;
mod block;
mod errors;
mod chain;

fn main() -> std::io::Result<()> {
    pretty_env_logger::try_init_timed().unwrap();

    return Ok(());
}
