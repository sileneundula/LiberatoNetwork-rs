use libp2p::autonat::Behaviour as AutonatBehaviour;
use libp2p::autonat::v2::client::Behaviour as AutonatClient;
use libp2p::autonat::v2::client::Config as AutonatClientConfig;
use libp2p::autonat::v2::server::Behaviour as AutonatServer;
use libp2p::floodsub::Behaviour as FloodSubBehaviour;
use libp2p::gossipsub::Behaviour as GossipSubBehaviour;
use libp2p::gossipsub::Config as GossipSubConfig;

use libp2p::identify::Behaviour as IdentifyBehaviour;
use libp2p::identify::Config as IdentifyConfig;

use libp2p::gossipsub::MessageAuthenticity;

use libp2p::relay::Behaviour as RelayBehaviour;
use libp2p::request_response::Behaviour as RequestResponseBehaviour;
use libp2p::kad::Behaviour as KademliaBehaviour;
use libp2p::kad::store::MemoryStore;
use libp2p::relay::client::Behaviour as RelayClient;
use libp2p::relay::Behaviour as RelayServer;
use libp2p::relay::Config as RelayServerConfig;
use libp2p::swarm::NetworkBehaviour;
use libp2p::mdns::Behaviour as MdnsBehaviour;

use libp2p::floodsub::Behaviour as FloodsubBehaviour;
use libp2p::floodsub::Config as FloodsubConfig;

use rand::rngs::OsRng;

use crate::networking::internals::keys::ShrineKeys;


pub mod peer_discovery;


#[derive(NetworkBehaviour)]
pub struct ShrineBehaviour {
    pub autonat_client: AutonatClient,
    pub autonat_server: AutonatServer,
    //pub gossipsub: GossipSubBehaviour,
    pub identify: IdentifyBehaviour,
    pub floodsub: FloodsubBehaviour,
    pub kademlia: KademliaBehaviour<MemoryStore>,
    pub relay_server: RelayServer,
    //request_response: RequestResponseBehaviour<FileExchangeCodec>,
}

impl ShrineBehaviour {
    pub fn new(key: ShrineKeys) -> Self {
        let peer_id = key.key.public().to_peer_id();
        
        let rng = OsRng;


        ShrineBehaviour {
            autonat_client: AutonatClient::new(rng, AutonatClientConfig::default()),
            autonat_server: AutonatServer::new(rng),
            
            // GOSSIP, FLOOD, IDENTIFY, KAD
            // TODO: Check unwrap for unexpected errors
            // 11/18-11/19: Cloudflare unwrap error (historical)
                //gossipsub: GossipSubBehaviour::new(MessageAuthenticity::Anonymous, GossipSubConfig::default()).expect("Failed"),
            identify: IdentifyBehaviour::new(IdentifyConfig::new(String::from("Shrindo-Identify"), key.key.public())),
            floodsub: FloodsubBehaviour::new(key.key.public().to_peer_id()),
            kademlia: KademliaBehaviour::new(key.key.public().to_peer_id(),MemoryStore::new(key.key.public().to_peer_id())), // Create a Memory Store Service For Gathering Peers
            relay_server: RelayServer::new(peer_id, RelayServerConfig::default())
        }
    }
}