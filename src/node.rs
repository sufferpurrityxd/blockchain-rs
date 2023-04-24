use futures::prelude::*;
use libp2p::{
    swarm,
    mdns,
};

use crate::{
    network::{
       BlockchainBehaviour,
        BlockchainBehaviourEvent,
        swarm_builder
    },
    storage::Storage,
    miner::Miner,
    user::User
};

pub struct Node {
    pub swarm: swarm::Swarm<BlockchainBehaviour>,
    pub storage: Storage,
    pub miner: Option<Miner>,
    pub user: Option<User>,
}


impl Node {
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
                _ => {}
            }
        }
    }
    }
}
