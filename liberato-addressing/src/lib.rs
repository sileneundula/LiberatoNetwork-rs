//! # Liberato Addressing Scheme
//! 
//! Liberato Addressing Scheme is a modular addressing scheme for the open internet.
//! 
//! It addresses length of Addresses as different, interpreted versions.
//! 
//! The address length is around `44`. `20-64``
//! 
//! ## Encoding
//! 
//! Addresses are derived from the following formats:
//! - [X] X59-fmt
//! - [X] DER
//! - [X] Bytes
//! - [X] PEM

use fixedstr::str32;
use fixedstr::str256;
use fixedstr::str64;
use fixedstr::str128;
use std::collections::HashSet;

pub trait AddressScheme {

}

pub struct LiberatoSchema {
    id: u64,
    schemes: HashSet<u8,str128>, // 28 - 64
    sources: HashSet<str128,>
}

pub struct LiberatoSchemaRegistar {
    description: str256,
}


/// # Liberato Address
/// 
/// Contains the:
/// - [X] Address as a str256
/// - [X] Prefix as str64.
/// - [X] Deriviation Method as str64
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct LiberatoAddress {
    addr: str256,
    prefix: str64,
    deriviation_method: str64,
}

pub struct LiberatoSlab {
    addr: LiberatoAddress,
    addr_derived_from: LiberatoAddressInfo,

}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct LiberatoAddressInfo {
    derived_from: str64,
    derived_from_hash_id: Option<str128>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct LiberatoAddressWithData {
    address: LiberatoAddress,
    
    network: str128,
    actions: [str32;10],
}
