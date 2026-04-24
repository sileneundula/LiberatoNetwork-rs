use fixedstr::str64;

use crate::errors::OpenAddressErrors;

pub trait OpenAddressRequired: Sized {
    fn derive_from_str<T: AsRef<str>>(s: T) -> Result<Self,OpenAddressErrors>;
    fn derive_from_bytes<T: AsRef<[u8]>>(s: T) -> Result<Self,OpenAddressErrors>;
}

pub trait Network {
    fn default_network() -> str64;
}