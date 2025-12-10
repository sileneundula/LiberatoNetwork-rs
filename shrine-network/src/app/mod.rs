use log::{warn,info, trace, debug};
use log::error;
use pretty_env_logger;
use crate::networking::internals::keys;
use crate::networking::internals::swarm::ShrineSwarm;

use libp2p::kad::record::store::MemoryStore;

use libp2p::floodsub::Topic;

pub mod bootstrap;

const swarm_creation: &str = "0x22";
const SHRINDO_TOPIC: &str = "shrindo-0x20CB-v1";
const SHRINDO_TOPIC_BOOTSTRAP: &str = "shrindo-0x20CB-bootstrap-v1";
const SHRINDO_TOPIC_KEYEXCHANGE: &str = "shrindo-0x20CB-keyexchange";

pub struct ShrindoApp;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    debug!("Pretty Env Logger Initialized...")

    // Key Generation
    let key = crate::networking::internals::keys::ShrineKeys::generate_ed25519();
    let local_peer_id = key.key.public().to_peer_id();
    info!("Peer-ID: {}", peer_id);

    let mut swarm: libp2p::Swarm<crate::networking::internals::behavior::ShrineBehaviour> = ShrineSwarm::new(key);
    info!("Swarm Creation ID: {}", swarm_creation );

    // Store + Kademelia
    let store = MemoryStore::new(local_peer_id);
    let kademlia = Kademlia::new(local_peer_id, store);
    
    
    // Behaviour
    swarm.behaviour_mut().floodsub.subscribe(Topic::new(SHRINDO_TOPIC));
    swarm.behaviour_mut().floodsub.subscribe(Topic::new(SHRINDO_TOPIC_BOOTSTRAP));

    Ok(())


}