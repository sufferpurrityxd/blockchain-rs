pub mod command;

use either::Either;
use futures::{
  prelude::*,
  channel::mpsc::Sender,
  channel::mpsc::Receiver,
};

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

use crate::{
  storage::Storage,
  network::command::Command,
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
  pub storage: Storage,
  pub command_rx: Receiver<Command>,
}


impl NetworkLoop {
  pub fn new(
    swarm: swarm::Swarm<BlockchainBehaviour>,
    storage: Storage,
    command_rx: Receiver<Command>,
  ) -> Self {
    return Self {
      swarm,
      storage,
      command_rx,
    }
  }

  pub async fn handle_swarm_event(
    &mut self,
    event: swarm::SwarmEvent<
      BlockChainBehaviourEnum,
      Either<gossipsub::HandlerError, void::Void>,
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

  pub async fn handle_command(&mut self, command: Command) {
    match command {
      Command::AddBlock {
        key,
        block
      } => {
        match self.storage.add_item(key, &block) {
          Ok(_) => log::info!("Added new block into storage"),
          Err(e) => log::error!("Failed to add new block into storage, e: {e:?}")
        }
        match bincode::serialize(&block) {
          Ok(bytes_block) => {
              match self.swarm.behaviour_mut().gossipsub.publish(
                gossipsub::IdentTopic::new("BLOCK"),
                bytes_block,
              ) {
                Ok(_) => log::info!("Sent new block to peers"),
                Err(e) => log::error!("Failed to send new block to peers: {e:?}"),
              };
          }
          Err(e) => log::error!("Failed to serialize block, e: {e:?}"),
        }
      }
      _ => {},
    }
  }

  pub async fn execute(mut self) -> std::io::Result<()> {
    loop {
      futures::select! {
        event = self.swarm.next() => match event {
          Some(e) => self.handle_swarm_event(e).await,
          None => log::error!("Empty event from peer"),
        },
        command = self.command_rx.next() => match command {
          Some(command) => self.handle_command(command).await,
          None => log::error!("Empty command"),
        },
      }
    }
  }
}

pub async fn build(
  storage: Storage
) -> Result<(NetworkLoop, Sender<Command>), Box<dyn std::error::Error>> {
  let keypair = identity::Keypair::generate_ed25519();
  let peer_id = keypair.public().to_peer_id();
  let transactions_topic = gossipsub::IdentTopic::new("TRANSACTION");
  let blocks_topic = gossipsub::IdentTopic::new("BLOCK");

  let transport = tcp::async_io::Transport::default()
      .upgrade(core::upgrade::Version::V1)
      .authenticate(noise::NoiseAuthenticated::xx(&keypair)?)
      .multiplex(yamux::YamuxConfig::default())
      .boxed();
  let behaviour = {
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(keypair),
        gossipsub::ConfigBuilder::default()
          .max_transmit_size(MAX_TRANSMIT_SIZE)
          .build()?
    )?;
    let mdns = mdns::async_io::Behaviour::new(
      mdns::Config::default(),
      peer_id.clone(),
    )?;
    gossipsub.subscribe(&blocks_topic)?;
    gossipsub.subscribe(&transactions_topic)?;
    BlockchainBehaviour { gossipsub, mdns }
  };
  let mut swarm = swarm::SwarmBuilder::with_async_std_executor(
    transport,
    behaviour,
    peer_id,
  )
    .build();
  swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

  let (command_tx, command_rx) = futures::channel::mpsc::channel::<Command>(0);

  let network_loop = NetworkLoop::new(swarm, storage, command_rx);

  return Ok((network_loop, command_tx));
}
