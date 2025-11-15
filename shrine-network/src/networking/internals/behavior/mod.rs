use libp2p::autonat::Behaviour as AutonatBehaviour;
use libp2p::autonat::v2::client::Behaviour as AutonatClient;
use libp2p::autonat::v2::server::Behaviour as AutonatServer;
use libp2p::floodsub::Behaviour as FloodSubBehaviour;
use libp2p::gossipsub::Behaviour as GossipSubBehaviour;
use libp2p::identify::Behaviour as IdentifyBehaviour;
use libp2p::relay::Behaviour as RelayBehaviour;
use libp2p::request_response::Behaviour as RequestResponseBehaviour;
use libp2p::kad::Behaviour as KademliaBehaviour;
use libp2p::kad::store::MemoryStore;
use libp2p::relay::client::Behaviour as RelayClient;
use libp2p::relay::Behaviour as RelayServer;
use libp2p::swarm::NetworkBehaviour;


#[derive(NetworkBehaviour)]
pub struct ShrineBehaviour {
    autonat_client: AutonatClient,
    autonat_server: AutonatServer,
    gossipsub: GossipSubBehaviour,
    identify: IdentifyBehaviour,
    kademlia: KademliaBehaviour<MemoryStore>,
    relay_client: RelayClient,
    relay_server: RelayServer,
    //request_response: RequestResponseBehaviour<FileExchangeCodec>,
}