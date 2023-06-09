pub mod actions;
pub mod miner;

use either::Either;
use futures::{
  prelude::*,
  channel::mpsc::{
    Sender,
    Receiver,
  },
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
  network::actions::{
    Event,
    Command,
  },
};
use crate::chain::block::Block;
use crate::network::actions::GossipsubEvent;

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
  pub event_tx: Sender<Event>,
}


impl NetworkLoop {
  pub fn new(
    swarm: swarm::Swarm<BlockchainBehaviour>,
    storage: Storage,
    command_rx: Receiver<Command>,
    event_tx: Sender<Event>,
  ) -> Self {
    return Self {
      swarm,
      storage,
      command_rx,
      event_tx,
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
      swarm::SwarmEvent::Behaviour(
        BlockChainBehaviourEnum::Gossipsub(
          gossipsub::Event::Message {
            propagation_source: _peer_id,
            message_id: _id,
            message,
          },
        ),
      ) => self.handle_gossipsub_message(message).await,
      swarm::SwarmEvent::NewListenAddr { address, .. } => {
        log::info!("Local node is running on: {address}");
      },
      _ => {}
    }
  }

  pub async fn handle_command(&mut self, command: Command) {
    match command {
      Command::AddBlock(meta) => {
        self.add_block(meta.key, &meta.block).await;
        match bincode::serialize(&GossipsubEvent::SyncNetworkBlock(meta)) {
          Ok(event) => {
              match self.swarm.behaviour_mut().gossipsub.publish(
                gossipsub::IdentTopic::new("BLOCK"),
                event.as_slice(),
              ) {
                Ok(_) => log::info!("Sent new block to peers"),
                Err(e) => log::error!("Failed to send new block to peers: {e:?}"),
              };
          }
          Err(e) => log::error!("Failed to serialize block, e: {e:?}"),
        }
      }
      Command::SendTransaction(meta) => {
        match bincode::serialize(&GossipsubEvent::SendNetworkTransaction(meta))  {
          Ok(event) => {
            match self.swarm.behaviour_mut().gossipsub.publish(
              gossipsub::IdentTopic::new("TRANSACTION"),
              event.as_slice(),
            ) {
              Ok(_) => log::info!("Sent transaction to peers"),
              Err(e) => log::error!("Failed to send new transaction to peers: {e:?}"),
            }
          }
          Err(e) => log::error!("Failed to serializer transaction, e: {e:?}"),
        }
      },
    }
  }

  pub async fn handle_gossipsub_message(&mut self, message: gossipsub::Message) {
    match bincode::deserialize::<GossipsubEvent>(message.data.as_slice()) {
      Ok(meta) => match meta {
        GossipsubEvent::SyncNetworkBlock(meta)=> {
          match self.storage.get_block(meta.key) {
            Some(_) => log::info!("Block with index: {:?} already exists", meta.key),
            None => {
              self.add_block(meta.key, &meta.block).await;
              self.event_tx.send(Event::SyncBlock(meta)).await.unwrap();
            },
          };
        },
        GossipsubEvent::SendNetworkTransaction(meta)  => {
          self.event_tx.send(Event::SendTransaction(meta)).await.unwrap();
        }
      }
      Err(e) => log::error!("Failed to deserialize message from gossipsub, e: {e:?}"),
    }
  }

  pub async fn execute(mut self) -> std::io::Result<()> {
    // Run network
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

  pub async fn add_block(&mut self, key: i32, block: &Block) {
    // Add new block into Storage
    match self.storage.add_item(key, block) {
      Ok(_) => log::info!("Added new block into storage: {block:?}"),
      Err(e) => log::error!("Failed to add new block into storage, e: {e:?}"),
    };
  }
}

pub async fn build(
  storage: Storage
) -> Result<(NetworkLoop, Sender<Command>, Receiver<Event>), Box<dyn std::error::Error>> {
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
  let (event_tx, event_rx) = futures::channel::mpsc::channel::<Event>(0);
  let network_loop = NetworkLoop::new(swarm, storage, command_rx, event_tx);

  return Ok((network_loop, command_tx, event_rx));
}
