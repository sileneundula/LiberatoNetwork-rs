//! # Muscarine Types | LiberatoNetwork Types
//! 
//! ## Description
//! 
//! This module contains common types used in the muscarine ecosystem.
//! 
//! ## Types
//! 
//! - [X] PeerAddr (The PeerAddr used to find peers)
//!     - [ ] BLAKE2B(28)
//! 
//! ### Keys
//! 
//! - [ ] X59Certificate
//!     - [ ] ED25519
//!     - [ ] ED448
//!     - [ ] Secp256k1
//!     - [ ] ECDSA
//!     - [ ] ShulginSigning (Hybrid)
//!     - [ ] DualFalcon
//! - [ ] Public Key
//! - [ ] Secret Key
//! - [ ] Keypair
//! 
//! ### PeerAddr
//! 
//! The PeerAddr is a commonly used, modular format for finding peers that uses cryptography to ensure certain properties for the peer address.
//! 
//! It uses a fixed string of 256 bytes for an address, making it more than useable in real-world applications.
//! 
//! It uses several different formats.

use fixedstr::str128;
use fixedstr::str256;
use fixedstr::str64;
use fixedstr::str32;
use core::str::FromStr;
use std::fmt::Error;
use serde::{Serialize,Deserialize};

#[no_std]

pub mod prelude;

pub trait Address {
    /// # Lookup Prefix (str64)
    fn lookup_prefix(&self);
    /// # As Prefix
    fn as_prefix(&self) -> str64;
}

pub trait FromPublicKey {
    fn from_keypair() -> Self;
    fn from_pk() -> Self;
}

pub trait IntoPublicKey {
    fn export_to_x59(&self) -> Self;
}

pub trait ToAddress {
    fn into_address<T: AsRef<str>>(_type: PeerAddrType, input: T) -> PeerAddr;
}

pub trait PeerAddressFormat {
    fn as_bytes(&self) -> &[u8];
    fn into_bytes(&self) -> Vec<u8>;

    fn as_hexadecimal(&self) -> str256;
    fn as_base58(&self) -> str256;
    fn as_base32(&self) -> str256;

    fn is_encoded(&self) -> bool;
}

/// # PeerAddr
/// 
/// A `PeerAddr` is a string of up to 256 bytes.
/// 
/// The `Prefix` is a 64-byte string
#[derive(Clone,Copy,PartialEq,PartialOrd,Hash,Debug,Serialize,Deserialize)]
pub struct PeerAddr {
    /// Address
    pub addr: str256,
    /// Prefix 
    pub prefix: str64,
    /// Extension
    pub extension: Option<(str32,u32)>,
}

impl PeerAddr {
    pub fn new<T: AsRef<str>>(addr: T, prefix: T) -> Result<Self, &'static str> {
        let address = str256::from_str(addr.as_ref())?;
        let prefix = str64::from_str(prefix.as_ref())?;
        
        return Ok(Self {
            addr: address,
            prefix: prefix,
            extension: None,
        })
    }
}

impl Default for PeerAddr {
    fn default() -> Self {
        Self {
            addr: str256::from_str("DefaultAddress").unwrap(),
            prefix: str64::from_str("Example").unwrap(),
            extension: None,
        }
    }
}

/// # PeerAddrType
/// 
/// A `PeerAddrType` is the type of peer address used.
pub struct PeerAddrType(pub u32, pub u8);

pub mod keys;
pub mod certificate;

pub struct HashAddr(pub str128);

impl HashAddr {
    pub fn new<T: AsRef<str>>(s: T) -> core::result::Result<Self,Errors> {
        let x = str256::from_str(s.as_ref());

        match x {
            Ok(v) => return Ok(Self(v)),
            Err(_) => return Err(Errors::HashAddrError),
        }
    }
}

pub enum Errors {
    PeerAddrError,
    HashAddrError
}
