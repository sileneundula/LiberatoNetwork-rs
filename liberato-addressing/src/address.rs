//! # Address Scheme
//! 
//! Uses blake2b(variable digest) for different purposes defined as `source_id: u32`

use fixedstr::{str32,str64, str128, str256};

pub struct MuscarineAddress {
    pub address: str256,
}

pub struct MuscarineAddressData {
    //=====required======//
    pub source_id: u64,
    pub source_checksum: str32, // 8-15
}


/// # LiberatoAddress
pub struct LiberatoAddressInternals {
    source_id: u32,
    
    address: str256,
    
    address_derived_from: str64,
    input_location: str256,
}

pub struct LiberatoAddressSource {
    cn: str128, // common_name, easy-identifier
    
    unique_id: str128, // 48-bytes (Hex)
    unique_id_fingerprint: str32, // {8-15} bytes | Hash of Unique_ID
    network: str256,
}

pub struct LiberatoAddressMapping()