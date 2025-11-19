//! # networking/internals/keys
//! 
//! Simple Wrapper Around `libp2p::identity::Keypair`
//! 
//! TODO: Add RSA4096
//! 
//! ## Types of Keys:
//! - [X] ED25519
//! - [X] ECDSA
//! - [X] SECP256k1
//! - [ ] RSA
//! 
//! ## Exporting
//! 
//! - [X] Protobuf
//!     - [X] To
//!     - [X] From

use libp2p::identity::*;

/// # ShrineKeys
/// 
/// Keypairs for usage.
#[derive(Clone)]
pub struct ShrineKeys {
    pub key: Keypair
}

impl ShrineKeys {
    pub fn from(protobuf: &[u8]) -> Result<Keypair,DecodingError> {
        Keypair::from_protobuf_encoding(protobuf)
    }
    pub fn generate_ed25519() -> ShrineKeys {
        return Self {
            key: Keypair::generate_ed25519()
        }
    }
    pub fn generate_ecdsa() -> ShrineKeys {
        return Self {
            key: Keypair::generate_ecdsa()
        }
    }
    pub fn generate_secp256k1() -> ShrineKeys {
        return Self {
            key: Keypair::generate_secp256k1()
        }
    }
    pub fn into_protobuf(&self) -> Result<Vec<u8>,DecodingError>  {
        self.key.to_protobuf_encoding()
    }
}