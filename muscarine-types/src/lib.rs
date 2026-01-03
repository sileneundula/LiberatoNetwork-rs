//! # Muscarine Types
//! 
//! ## Description
//! 
//! This module contains common types used in the muscarine ecosystem.
//! 
//! ## Types
//! 
//! - [X] 

use fixedstr::str256;
use core::str::FromStr;

#[no_std]

pub trait Address {

}

/// # PeerAddr
/// 
/// A `PeerAddr` is a string of up to 256 bytes.
#[derive(Clone,Copy,PartialEq,PartialOrd,Hash,Debug)]
pub struct PeerAddr(pub str256);

pub struct HashAddr(pub str256);

impl PeerAddr {
    pub fn new<T: AsRef<str>>(s: T) -> core::result::Result<Self,Errors> {
        let x = str256::from_str(s.as_ref());

        match x {
            Ok(v) => return Ok(Self(v)),
            Err(_) => return Err(Errors::PeerAddrError),
        }
    }
}

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
