use fixedstr::str32;
use fixedstr::str256;
use fixedstr::str64;
use fixedstr::str128;
use heapless::Vec;

pub trait AddressScheme {

}


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct LiberatoAddress {
    addr: str256,
    prefix: str64,
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
