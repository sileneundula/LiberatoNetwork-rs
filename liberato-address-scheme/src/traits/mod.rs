use openinternetcryptographykeys::prelude::essentials::{OpenInternetCryptographySecretKey, Slug20Algorithm};

use crate::prelude::LiberatoIdentity;

pub trait DeriveLiberatoAddress {
    /// # Derive Liberato Address
    /// 
    /// Derives a Liberato Address from a Public Key in PEM format.
    /// 
    /// # Arguments
    /// 
    /// * `public_key` - Public Key in PEM format.
    /// * `extension` - Optional extension to append to the address.
    fn derive_liberato_address<T: AsRef<str>>(public_key: T, extension: Option<T>) -> String;
    fn derive_liberato_address_pk<T: AsRef<str>>(public_key: T) -> String;
    //fn derive_muscarine_address<T: AsRef<str>>(&self, public_key: T) -> String;
}

pub trait GenerateLiberatoIdentity: Sized {
    /// # Generate Liberato Identity
    /// 
    /// Generates a Liberato Identity from a Public Key in PEM format.
    /// 
    /// # Arguments
    /// 
    /// * `public_key` - Public Key in PEM format.
    fn generate_liberato_identity_with_extension<T: AsRef<str>>(alg: Slug20Algorithm, extension: T) -> LiberatoIdentity;
    fn generate_liberato_identity(alg: Slug20Algorithm) -> LiberatoIdentity;
}