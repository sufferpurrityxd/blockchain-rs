mod storage;
mod network;
mod chain;
mod node;

#[async_std::main]
async fn main() -> Result<(), Box< dyn std::error::Error>> {
  pretty_env_logger::init_timed();

  let storage = storage::Storage::new();
  let (network_loop, _, _) = network::build(storage).await?;

  async_std::task::spawn(network_loop.execute());

  return Ok(());
}
