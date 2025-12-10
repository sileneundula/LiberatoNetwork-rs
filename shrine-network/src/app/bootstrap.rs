use libp2p::PeerId;
use crate::errors::NetworkingStackErrors;
use log::{debug,error};

pub struct BootstrapAddresses {
    addresses: Vec<PeerId>,
}

impl BootstrapAddresses {
    pub fn new() {

    }
    pub fn add(&mut self, peer_id: PeerId) {
        self.addresses.push(peer_id)
    }
    pub fn remove(&mut self, peer_id: PeerId) -> Result<(),NetworkingStackErrors> {
        debug!("[Shrine::Networking::KAD][Bootstrap] Removing Peer ID");
        
        if self.addresses.contains(&peer_id) {
            debug!("[Shrine::Networking::KAD][Bootstrap] Peer ID is in bootstrap");
            let value: Result<usize, usize> = self.addresses.binary_search(&peer_id);
            debug!("[Shrine::Networking::KAD][Bootstrap] Found Peer ID");
            let x = self.addresses.swap_remove(value.unwrap());
            debug!("[Shrine::Networking::KAD][Bootstrap] Swapped Peer ID with Last Element");
            Ok(())
        }
        else {
            error!("[Error: 0x00] [Shrine::Networking::KAD] Removing Peer ID Failed");
            return Err(NetworkingStackErrors::Networking(0x00, String::from("KAD-BOOTSTRAP"), String::from("Failed To Remove Peer ID"), String::from("Check whether Peer ID was in Struct"), peer_id.to_string()))
        }
    }
}