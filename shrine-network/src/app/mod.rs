use libp2p::core::transport::ListenerId;
use libp2p::futures::StreamExt;
use log::{warn,info, trace, debug};
use log::error;
use pretty_env_logger;
use crate::networking::internals::keys;
use crate::networking::internals::swarm::ShrineSwarm;

use libp2p::floodsub::Topic;

use libp2p::swarm::SwarmEvent;

use libp2p::Multiaddr;

use std::str::FromStr;

use crate::core::internals::Utils;

use libp2p::swarm::ListenAddresses;

use dotenvy;
pub mod bootstrap;

const swarm_creation: &str = "0x22";
const SHRINDO_TOPIC: &str = "shrindo-0x20CB-v1";
const SHRINDO_TOPIC_BOOTSTRAP: &str = "shrindo-0x20CB-bootstrap-v1";
const SHRINDO_TOPIC_KEYEXCHANGE: &str = "shrindo-0x20CB-keyexchange";
const SHRINDO_TOPIC_ESSENTIALS: &str = "Lament-Essentials";

const local_address: &str = "/ip4/127.0.0.1/tcp/9076";

pub struct ShrindoApp;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();

    pretty_env_logger::init();
    debug!("Pretty Env Logger Initialized...");

    info!("Generating Ephermal ED25519 Key...");
    // Key Generation
    let key = crate::networking::internals::keys::ShrineKeys::generate_ed25519();
    let local_peer_id = key.key.public().to_peer_id();
    info!("Peer-ID: {}", local_peer_id);

    // Swarm
    let mut swarm: libp2p::Swarm<crate::networking::internals::behavior::ShrineBehaviour> = ShrineSwarm::new(key);
    info!("Swarm Creation ID: {}", Utils::create_swarm_id(local_peer_id).unwrap());

    //=====BOOTSTRAP CLI=====//


    // Store + Kademelia
    //let store = MemoryStore::new(local_peer_id);
    
    
    // Behaviour
    swarm.behaviour_mut().floodsub.subscribe(Topic::new(SHRINDO_TOPIC));
    info!("[Floodsub] Subscribed to {}", SHRINDO_TOPIC);
    swarm.behaviour_mut().floodsub.subscribe(Topic::new(SHRINDO_TOPIC_BOOTSTRAP));
    info!("[Floodsub] Subscribed to {}", SHRINDO_TOPIC_BOOTSTRAP);

    let id = swarm.listen_on(Multiaddr::from_str(local_address).unwrap())?;

    info!("[SHRINDO] Connecting to SHRINDO/SHRINE with Peer-ID: {}",local_peer_id);

    loop {
        /// # Event
        let event = swarm.select_next_some().await;

        match event {
            SwarmEvent::NewListenAddr { listener_id, address} => {
                info!("[Swarm] Listening On {} with listener_id: {}", local_address, listener_id);
            },
            _ => ()
        }
    }

    Ok(())


}

#[test]
fn run() {
    let x = main();
}