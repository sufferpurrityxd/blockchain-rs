mod storage;
mod network;
mod chain;
mod node;

#[async_std::main]
async fn main() -> Result<(), Box< dyn std::error::Error>> {
  pretty_env_logger::init_timed();

  let network_loop = network::build().await?;
  async_std::task::spawn(network_loop.execute());

  return Ok(());
}
