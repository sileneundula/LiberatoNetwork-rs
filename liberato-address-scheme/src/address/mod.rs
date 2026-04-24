pub enum LiberatoAddressScheme {
    OpenInternetCryptographyKeys(AddressOpenInternetCryptographyKeysTypes), // BLAKE2B (48)
    ContentID(AddressContentID),
}

pub enum AddressOpenInternetCryptographyKeysTypes {
    PEM,
    X59FMT,
}

pub enum AddressContentID {
    BYTES,
}