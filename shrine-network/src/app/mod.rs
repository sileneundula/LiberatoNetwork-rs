use libp2p::futures::StreamExt;
use log::{warn,info, trace, debug};
use log::error;
use pretty_env_logger;
use crate::networking::internals::keys;
use crate::networking::internals::swarm::ShrineSwarm;

use libp2p::floodsub::Topic;

use libp2p::swarm::SwarmEvent;

use crate::core::internals::Utils;

use dotenvy;
pub mod bootstrap;

const swarm_creation: &str = "0x22";
const SHRINDO_TOPIC: &str = "shrindo-0x20CB-v1";
const SHRINDO_TOPIC_BOOTSTRAP: &str = "shrindo-0x20CB-bootstrap-v1";
const SHRINDO_TOPIC_KEYEXCHANGE: &str = "shrindo-0x20CB-keyexchange";

pub struct ShrindoApp;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();

    pretty_env_logger::init();
    debug!("Pretty Env Logger Initialized...");

    // Key Generation
    let key = crate::networking::internals::keys::ShrineKeys::generate_ed25519();
    let local_peer_id = key.key.public().to_peer_id();
    info!("Peer-ID: {}", local_peer_id);

    // Swarm
    let mut swarm: libp2p::Swarm<crate::networking::internals::behavior::ShrineBehaviour> = ShrineSwarm::new(key);
    info!("Swarm Creation ID: {}", Utils::create_swarm_id(local_peer_id).unwrap());

    // Store + Kademelia
    //let store = MemoryStore::new(local_peer_id);
    
    
    // Behaviour
    swarm.behaviour_mut().floodsub.subscribe(Topic::new(SHRINDO_TOPIC));
    swarm.behaviour_mut().floodsub.subscribe(Topic::new(SHRINDO_TOPIC_BOOTSTRAP));

    /*
    loop {
        let event = swarm.select_next_some().await;

        match event {
            SwarmEvent::Behaviour(libp2p::kad::Event::OutboundQueryProgressed { id, result, stats, step } {

            })
        }
    }
    */

    Ok(())


}

#[test]
fn run() {
    let x = main();
}