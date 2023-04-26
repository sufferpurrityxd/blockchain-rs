use either::Either;
use futures::prelude::*;

use libp2p::{
  Transport,
  gossipsub,
  identity,
  swarm,
  noise,
  yamux,
  core,
  mdns,
  tcp,
};

const MAX_TRANSMIT_SIZE: usize = 262144;


#[derive(swarm::NetworkBehaviour)]
#[behaviour(out_event = "BlockChainBehaviourEnum")]
pub struct BlockchainBehaviour {
  pub gossipsub: gossipsub::Behaviour,
  pub mdns: mdns::async_io::Behaviour,
}

pub enum BlockChainBehaviourEnum {
  Gossipsub(gossipsub::Event),
  Mdns(mdns::Event),
}

impl From<gossipsub::Event> for BlockChainBehaviourEnum {
  fn from(value: gossipsub::Event) -> Self {
    return BlockChainBehaviourEnum::Gossipsub(value);
  }
}

impl From<mdns::Event> for BlockChainBehaviourEnum {
  fn from(value: mdns::Event) -> Self {
    return BlockChainBehaviourEnum::Mdns(value);
  }
}


pub struct NetworkLoop {
  pub swarm: swarm::Swarm<BlockchainBehaviour>,
}


impl NetworkLoop {
  pub fn new(swarm: swarm::Swarm<BlockchainBehaviour>) -> Self {
    return Self {
      swarm,
    }
  }

  pub async fn handle_swarm_event(
    &mut self,
    event: swarm::SwarmEvent<
      BlockChainBehaviourEnum,
      Either<gossipsub::HandlerError, void::Void>
    >,
  ) {
    match event {
      swarm::SwarmEvent::Behaviour(
        BlockChainBehaviourEnum::Mdns(
          mdns::Event::Discovered(
            list,
          )
        )
      ) => {
        for (peer_id, _) in list {
          log::info!("New peer: {peer_id}");
          self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
        }
      },
      swarm::SwarmEvent::Behaviour(
        BlockChainBehaviourEnum::Mdns(
          mdns::Event::Expired(
            list,
          )
        )
      ) => {
        for (peer_id, _) in list {
          log::info!("Peer has expired: {peer_id}");
          self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
        }
      },
      swarm::SwarmEvent::NewListenAddr { address, .. } => {
        log::info!("Local node is running on: {address}");
      }
      _ => {}
    }
  }

  pub async fn execute(mut self) -> std::io::Result<()> {
    loop {
      futures::select! {
        event = self.swarm.next() => self.handle_swarm_event(event.expect("1")).await,
      }
    }
  }
}

pub async fn build() -> Result<NetworkLoop, Box<dyn std::error::Error>> {
  let keypair = identity::Keypair::generate_ed25519();
  let peer_id = keypair.public().to_peer_id();

  let mut swarm = swarm::SwarmBuilder::with_async_std_executor(
    tcp::async_io::Transport::default()
      .upgrade(core::upgrade::Version::V1)
      .authenticate(noise::NoiseAuthenticated::xx(&keypair)?)
      .multiplex(yamux::YamuxConfig::default())
      .boxed(),
    BlockchainBehaviour {
      gossipsub: gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(keypair),
        gossipsub::ConfigBuilder::default()
          .max_transmit_size(MAX_TRANSMIT_SIZE)
          .build()?
      )?,
      mdns: mdns::async_io::Behaviour::new(
        mdns::Config::default(),
        peer_id.clone())?
    },
    peer_id,
  )
    .build();
  swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
  let network_loop = NetworkLoop::new(swarm);

  return Ok(network_loop);
}