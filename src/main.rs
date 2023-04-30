mod storage;
mod network;
mod chain;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init_timed();
  let storage = storage::Storage::new();
  let blockchain = chain::blockchain::Blockchain::new(storage.get_chain());
  let (network_loop,
    command_tx,
    event_rx,
  ) = network::build(storage).await?;
  let miner = network::miner::Miner::new(blockchain, command_tx.clone(), event_rx);
  async_std::task::spawn(network_loop.execute());
  async_std::task::spawn(miner.execute());
  loop {}
}
