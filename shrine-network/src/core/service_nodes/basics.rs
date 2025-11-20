//! # Basics of Service Nodes
//! 
//! - [ ] Node Information
//!     - [ ] ShulginSigning
//!     - [ ] WOTS Signing
//!     - [ ] * Idea like the one i made for decentralized databases using ZKPs
//! 
//! TODO: Add an Easy String type Enum that allows for different types based on specified inputs as a library like for fingerprint it should be Blake2b(28) or something

use std::collections::HashMap;

use crate::core::crx::UserCertificate;
use crate::core::crx::UserCertificatePriv;


pub struct NodeInformation {
    id: u64,
    certificate: UserCertificate,
}

/// # NodeInformation (Node's Certificate and Info)
pub struct NodePrivateInfo {
    id: u64,
    local_id: u64,
    certificate: UserCertificatePriv,
}

pub struct ShrineDomain {
    tld: String,
    name: String,
    
    subdomains: Option<Vec<String>>,
}

pub struct ServiceAssignedDatabase {

}

impl NodePrivateInfo {
    pub fn generate(local_id: u64, id: u64) -> Self {
        let cert = UserCertificatePriv::generate();

        return Self {
            id: id,
            local_id: local_id,
            certificate: cert,
        }
    }
    pub fn pretty_print(&self) -> String {
        let x = self.certificate.publiccert();
        return x.export().unwrap()
    }
}

#[test]
fn run() {
    let node_slab = NodePrivateInfo::generate(0, 0);
    println!("{}",node_slab.pretty_print())

}