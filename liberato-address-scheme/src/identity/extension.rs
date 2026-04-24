use std::str::FromStr;

use fixedstr::str128;

pub struct LiberatoSmartExtensions {
    pub data: String,
    pub checksum: str128,
}

pub struct LiberatoExtensions {
    pub address_extension: str128,
}

impl LiberatoExtensions {
    pub fn new<T: AsRef<str>>(s: T) -> LiberatoExtensions {
        return LiberatoExtensions { 
            address_extension: str128::from_str(s.as_ref()).unwrap(), 
        }
    }
}