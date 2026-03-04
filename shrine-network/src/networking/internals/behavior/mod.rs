//! # Peyote-Network
//! 
//! Peyote Network is a libp2p network for use in distributed/decentralized systems.
//! 
//! ## Features
//! 
//! - [X] Implemented Protocols
//!     - [ ] AUTONAT (Client + Server)
//!     - [ ] GossipSub (Broadcasting Data to a certain SHA256 topic)
//!     - [ ] FloodSub
//!     - [ ] Peer Routing
//!         - [X] Kademelia (DHT)
//!         - [X] MDNS (local network)
//!     

use libp2p::autonat::Behaviour as AutonatBehaviour;
use libp2p::autonat::v2::client::Behaviour as AutonatClient;
use libp2p::autonat::v2::client::Config as AutonatClientConfig;
use libp2p::autonat::v2::server::Behaviour as AutonatServer;
use libp2p::gossipsub::Behaviour as GossipSubBehaviour;
use libp2p::gossipsub::Config as GossipSubConfig;
#[macro_use(NetworkBehaviour)]
use libp2p::swarm::NetworkBehaviour;


use libp2p::identify::Behaviour as IdentifyBehaviour;
use libp2p::identify::Config as IdentifyConfig;

use libp2p::gossipsub::MessageAuthenticity;

use libp2p::gossipsub::ConfigBuilder as GossipConfigBuilder;
use libp2p::gossipsub::ConfigBuilderError as GossipConfigBuilderError;

use libp2p::request_response::Behaviour as RequestResponseBehaviour;
use libp2p::kad::Behaviour as KademliaBehaviour;
use libp2p::kad::store::MemoryStore;
use libp2p::relay::client::Behaviour as RelayClient;
use libp2p::relay::Behaviour as RelayServer;
use libp2p::relay::Config as RelayServerConfig;
use libp2p::mdns::Behaviour as MdnsBehaviour;

use libp2p::floodsub::Behaviour as FloodsubBehaviour;
use libp2p::floodsub::Config as FloodsubConfig;

use libp2p::ping::Behaviour as PingBehaviour;
use libp2p::ping::Config as PingConfig;

// Must Use
use libp2p::gossipsub::Sha256Topic;

use rand::rngs::OsRng;

use crate::networking::internals::keys::P2PKeys;


pub mod peer_discovery;
pub mod topic;

#[derive(NetworkBehaviour)]
pub struct MuscarineBehaviour {
    pub autonat_client: libp2p::autonat::v2::client::Behaviour,
    pub autonat_server: libp2p::autonat::v2::server::Behaviour,

    // Ping (P2P)
    pub ping: PingBehaviour,

    // PubSub
    pub gossipsub: libp2p::gossipsub::Behaviour,
    //floodsub: FloodsubBehaviour,

    // Identify
    pub identify: IdentifyBehaviour,
    
    // Peer Routing
    pub kademlia: libp2p::kad::Behaviour<MemoryStore>,

    //pub floodsub: FloodsubBehaviour,

    // Relay
    pub relay_server: libp2p::relay::Behaviour,
    //request_response: RequestResponseBehaviour<FileExchangeCodec>,
}

impl MuscarineBehaviour {
    pub fn new(key: P2PKeys) -> Self {
        let peer_id = key.key.public().to_peer_id();
        
        let rng = OsRng;


        Self {
            autonat_client: AutonatClient::new(rng, AutonatClientConfig::default()),
            autonat_server: AutonatServer::new(rng),
            ping: PingBehaviour::new(PingConfig::default()),
            // TODO: remove unwrap
            gossipsub: GossipSubBehaviour::new(MessageAuthenticity::Signed(key.key.clone()), GossipConfigBuilder::default().validation_mode(libp2p::gossipsub::ValidationMode::Strict).build().unwrap()).unwrap(),
            identify: IdentifyBehaviour::new(IdentifyConfig::new(String::from("Shrindo-Identify"), key.key.clone().public())),
            //floodsub: FloodsubBehaviour::new(key.key.public().to_peer_id()),
            kademlia: KademliaBehaviour::new(key.key.public().to_peer_id(),MemoryStore::new(key.key.public().to_peer_id())), // Create a Memory Store Service For Gathering Peers
            relay_server: RelayServer::new(peer_id, RelayServerConfig::default())
        }
    }
}