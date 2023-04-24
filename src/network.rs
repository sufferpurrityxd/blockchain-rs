use libp2p::{
    Transport,
    gossipsub,
    identity,
    PeerId,
    noise,
    yamux,
    swarm,
    core,
    mdns,
    tcp,
};

#[derive(swarm::NetworkBehaviour)]
#[behaviour(out_event = "BlockchainBehaviourEvent" )]
pub struct BlockchainBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::async_io::Behaviour,
}

pub enum BlockchainBehaviourEvent {
    Gossipsub(gossipsub::Event),
    Mdns(mdns::Event),
}

impl From<gossipsub::Event> for BlockchainBehaviourEvent {
    fn from(value: gossipsub::Event) -> Self {
        return BlockchainBehaviourEvent::Gossipsub(value)
    }
}

impl From<mdns::Event> for BlockchainBehaviourEvent {
    fn from(value: mdns::Event) -> Self {
        return BlockchainBehaviourEvent::Mdns(value);
    }
}

pub fn swarm_builder() -> Result<swarm::Swarm<BlockchainBehaviour>, Box<dyn std::error::Error>> {
    let keypair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());
    let gossipsub_topic = gossipsub::IdentTopic::new("blockchain");
    log::info!("PeerID: {peer_id:?}");

    let transport = {
        let noise_config = noise::NoiseAuthenticated::xx(&keypair).unwrap();
        let yamux_config = yamux::YamuxConfig::default();

        tcp::async_io::Transport::new(tcp::Config::default()
            .nodelay(true))
            .upgrade(core::upgrade::Version::V1Lazy)
            .authenticate(noise_config)
            .multiplex(yamux_config)
            .timeout(std::time::Duration::from_secs(20))
            .boxed()
    };

    let mut swarm = {
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .max_transmit_size(262144)
            .build()
            .unwrap();
        let mut behaviour = BlockchainBehaviour {
            gossipsub: gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(keypair.clone()),
                gossipsub_config,
            )
                .unwrap(),
            mdns: mdns::Behaviour::new(
                mdns::Config::default(),
                peer_id.clone(),
            )
                .unwrap(),
        };
        log::debug!("Strated in topic: {gossipsub_topic:?}");
        behaviour.gossipsub.subscribe(&gossipsub_topic).unwrap();
        swarm::SwarmBuilder::with_async_std_executor(transport, behaviour, peer_id).build()
    };
    return Ok(swarm);
}
