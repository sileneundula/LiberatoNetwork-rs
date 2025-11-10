use libp2p::autonat::Behaviour as AutonatBehaviour;
use libp2p::floodsub::Behaviour as FloodSubBehaviour;
use libp2p::gossipsub::Behaviour as GossipSubBehaviour;
use libp2p::identify::Behaviour as IdentifyBehaviour;
use libp2p::relay::Behaviour as RelayBehaviour;
use libp2p::request_response::Behaviour as RequestResponseBehaviour;
use libp2p::kad::Behaviour as KademliaBehaviour;

#[derive(NetworkBehaviour)]
pub struct ShrineBehaviour {
    autonat_client: Toggle<AutonatClient>,
    autonat_server: Toggle<AutonatServer>,
    connection_limits: ConnectionLimits,
    dcutr: Toggle<Dcutr>,
    gossipsub: GossipSubBehaviour,
    identify: IdentifyBehaviour,
    kademlia: Toggle<Kademlia<MemoryStore>>,
    memory_connection_limits: MemoryConnectionLimits,
    //relay_client: Toggle<RelayClient>,
    //relay_server: Toggle<RelayServer>,
    request_response: RequestResponseBehaviour<FileExchangeCodec>,
}