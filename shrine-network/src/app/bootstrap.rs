use libp2p::PeerId;
use crate::errors::NetworkingStackErrors;
use log::{debug,error};
use blake2_rfc::blake2s::Blake2s;
use slugencode::SlugEncodingUsage;

/// # Bootstrap Addresses
/// 
/// The Bootstrap Addresses contains the `checksum` and `addresses`.
/// 
/// These addresses are used to connect to the network and bootstrap the process.
/// 
/// This struct is modular and can be used to bootstrap for many purposes.
pub struct BootstrapAddressesModular {
    checksum: String,
    addresses: Vec<PeerId>,
    bootstrap_usage: String,
}

impl BootstrapAddressesModular {
    pub fn new() -> Self {
        return Self {
            checksum: String::new(),
            addresses: vec![],
            bootstrap_usage: String::new(),
        }
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
    pub fn get_addresses(&self) -> Vec<PeerId> {
        return self.addresses.clone()
    }
    pub fn get_addresses_str(&self) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        
        for x in &self.addresses {
            output.push(x.to_base58())
        }
        return output
    }
    /// # Integrity Check
    /// 
    /// 8-byte BLAKE2s hash of the Peer IDs encoded in base58 to check for changes.
    pub fn integrity_check(&mut self) {
        let mut checksum_output = Blake2s::new(8);

        for i in &self.addresses {
            let x = i.to_bytes();
            checksum_output.update(&x);
        }

        let output = checksum_output.finalize();
        let encoder = SlugEncodingUsage::new(slugencode::SlugEncodings::Base58);
        let output_to_checksum = encoder.encode(output.as_bytes()).unwrap();
        self.checksum = output_to_checksum
    }
}