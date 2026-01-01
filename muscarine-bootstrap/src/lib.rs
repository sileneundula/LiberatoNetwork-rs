use fixedstr::str32;
use fixedstr::str128;
use fixedstr::str64;
use fixedstr::str256;

use serde::{Serialize,Deserialize};
use serde_big_array::BigArray;


/// # Modular Bootstrap
/// 
/// ## Description
/// 
/// A modular way of bootstrapping P2P nodes. Extendable by nature.
/// 
/// ## Bootstrapping Protocols
/// 
/// - [ ]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ModularBootstrap {
    bootstrap_protocol: str64,
    
    #[serde(with = "BigArray")]
    addresses: [str128;256],

    checksum: str32,
}

impl ModularBootstrap {
    /// # ModularBootstrap
    /// 
    /// This method creates a new modular bootstrap node with an array of 256 str128 that can be converted to the needed peers.
    /// 
    /// ## Modular Bootstrapping: Supporting Modules
    /// 
    /// This contains the list of supported bootstrapping implementations.
    /// 
    /// - [X] Peer ID (libp2p)
    ///     - [X] ED25519
    ///     - [X] ECDSA
    ///     - [X] Secp256k1
    ///     - [X] RSA
    /// - [ ] other
    pub fn new<T: AsRef<str>>(bootstrap_protocol: BootstrapProtocol, custom_network: T) {
        match bootstrap_protocol {
            BootstrapProtocol::InitFromDomain(v) => {

            }
            BootstrapProtocol::InitFromFile(v) => {

            }
            BootstrapProtocol::InitFromPeer(v) => {

            }
            BootstrapProtocol::InitFromPeers(v) => {

            }
            BootstrapProtocol::InitFromPeersFull(v) => {

            }
            BootstrapProtocol::InitializeRequest(v) => {

            }
        }
    }
}


pub struct BootstrapSignature {
    signature: str256,
    
    // Signature Type + Algorithm
    signature_type: BootstrapSignatureType,
    algorithm: str64,
}

pub enum BootstrapSignatureType {
    IsSignature,
    IsDigestofSignature(str32),
}


impl ModularBootstrap {
    pub fn empty() -> Self {
        Self {
            bootstrap_protocol: str64::new(),
            addresses: [str128::new();256],
            checksum: str32::new(),
        }
    }
}

/// # Bootstrap Protocol
/// 
/// Lists the methods of Bootstrapping Nodes For Muscarine-Network :TEMP
/// 
/// ## Protocols Implemented
/// 
/// - [ ] InitializeRequest
/// - [ ] InitFromPeer (contains 1 peer)
/// - [ ] InitFromPeers (contains 32 peers)
/// - [ ] InitFromPeersFull (contains 256 peers)
/// - [ ] InitFromDomain (str256 domain)
/// - [ ] InitFromFile (str256 path)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BootstrapProtocol {
    InitializeRequest(u32),
    
    // From Peers
    InitFromPeer(str128),

    /// InitFromPeers holds an array of 32 str128
    InitFromPeers([str128;32]),
    
    #[serde(with = "BigArray")]
    InitFromPeersFull([str128;256]),

    // From Domain
    InitFromDomain(str256),

    // From File
    InitFromFile(str256),

    // Search
    //SearchForPeers(str256),
}