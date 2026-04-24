use std::str::FromStr;

use fixedstr::{str64, str256};

/// # OpenAddressScheme
/// 
/// ## Hash Function
/// 
/// Blake2b(32-64): 32 entries
/// 
/// Blake2s(20-32): 12 entries
/// 
/// ## Values
/// 
/// 64 bytes: `Secure-Data`
/// 
/// 48 bytes: `Static-Content`
/// 
/// 40 bytes: `Key`
/// 
/// 20 bytes: `Abstraction-For-Identifiers`
pub struct OpenAddressScheme {
    pub address: str256,
    pub network_address_scheme_id: Option<str64>, // 8-20 bytes
    
    pub payload: Option<str256>,
}

impl OpenAddressScheme {
    pub fn new<T: AsRef<str>>(address: T, network_address_scheme_id: Option<T>, payload: Option<T>) -> Self {
        return Self {
            address: str256::from_str(address.as_ref())
            network_address_scheme_id: network_address_scheme_id
        }
    }
}