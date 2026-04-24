#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash)]
pub enum LiberatoAddressError {
    InvalidLiberatoAddress,
    InvalidLiberatoAddressExtension,
    InvalidLiberatoAddressPublicKey,
    InvalidLiberatoAddressPublicKeyExtension,
    Unknown,
} // Error