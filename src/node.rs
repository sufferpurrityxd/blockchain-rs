use std::cmp::min;
use std::os::unix::raw::blkcnt_t;
use std::sync::{Arc, Mutex};
use futures::prelude::*;
use libp2p::{
    swarm,
    mdns,
    gossipsub,
};

use crate::{
    network::{
       BlockchainBehaviour,
        BlockchainBehaviourEvent,
        swarm_builder
    },
    node::gossipsub::Message,
    storage::Storage,
    miner::Miner,
    user::User
};
use crate::block::Block;
use crate::tx::Tx;

pub struct Node {
    pub swarm: swarm::Swarm<BlockchainBehaviour>,
    pub storage: Storage,
    pub miner: Option<Miner>,
}


impl Node {
    pub async fn process_gossipsub_new_message(&mut self, message: Message) {
        if let Ok(block) = bincode::deserialize::<Block>(&message.data.as_slice()) {
            if let Some(miner) = self.miner.as_mut() {
                self.storage.save_block(&block);
                miner.blockchain.add_block(block);
            }

        }
        if let Ok(tx) = bincode::deserialize::<Tx>(&message.data.as_slice()) {
            if let Some(miner) = self.miner.as_mut() {
                miner.blockchain.add_unsigned_tx(tx);
            }
        };
    }

    pub async fn run_node(&mut self) {
        loop {
            futures::select! {
                event = self.swarm.select_next_some() => match event {
                    swarm::SwarmEvent::Behaviour(BlockchainBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    swarm::SwarmEvent::Behaviour(BlockchainBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    swarm::SwarmEvent::Behaviour(BlockchainBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => {
                        self.process_gossipsub_new_message(message).await

                    },
                _ => {}
                }
            }
        }
    }
}
