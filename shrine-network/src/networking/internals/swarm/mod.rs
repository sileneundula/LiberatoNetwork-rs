//! # Swarms
//! 
//! These are the following swarms that are provided for the protocol.
//! 
//! - [X] Core Protocol Swarm (SHRINESWARM)
//! - [ ] Bootstrap Swarm
//!     - [ ] Bootstraps Nodes using KAD and gateways
//!     - [ ] Port 9670
//! - [ ] Service
//! 

use libp2p::swarm::Config;
use libp2p::{PeerId, Swarm};
use super::behavior::ShrineBehaviour;
use libp2p::identity;
use super::transport::ShrineTransport;
use super::keys::ShrineKeys;

pub struct ShrineSwarm;

impl ShrineSwarm {
    pub fn new(key: ShrineKeys) -> Swarm<ShrineBehaviour> {
        let transport = ShrineTransport::new(key.clone().key);

        let pk = key.clone().key.public();
        let peer_id = pk.to_peer_id();

        let swarm = Swarm::new(transport.0,ShrineBehaviour::new(key.clone()), peer_id, Config::with_tokio_executor());

        return swarm


    }
}

/*
pub struct BootstrapSwarm;

impl BootstrapSwarm {
    pub fn new(key: ShrineKeys) -> Swarm<BootstrapSwarm> {
        let transport = ShrineTransport::new(key.clone().key);

        let pk = key.clone().key.public();
        let peer_id = pk.to_peer_id();

        let swarm = Swarm::new(transport.0,ShrineBehaviour::new(key.clone()), peer_id, Config::with_tokio_executor());

        return swarm
    }
}
    */