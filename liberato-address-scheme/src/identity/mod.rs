//! # Liberato Identity
//! 
//! An identity is defined as the following:
//! 
//! - [X] Public Key
//! - [X] Private Key
//! - [X] Public Address
//! - [X] Private Address

use std::str::FromStr;

use crate::traits::DeriveLiberatoAddress;
use crate::traits::GenerateLiberatoIdentity;
use openinternetcryptographykeys::prelude::essentials::*;
use openinternetcryptographykeys::prelude::essentials::{OpenInternetFromPemAny,OpenInternetFromStandardPEM,OpenInternetGeneration,OpenInternetIntoStandardPEM,OpenInternetSigner,OpenInternetVerifier};
use blake2_rfc::blake2b::Blake2b;
use fixedstr::str192;
use serde::Deserialize;
use serde::Serialize;
use bincode;
use pem::Pem;
use crate::identity::signature::LiberatoSignature;
use crate::identity::signature::LiberatoSignatureWithMessage;


use crate::prelude::CipherSuite;

/// Cipher Suite
pub mod cipher_suite;

/// Extensions To Liberato Identity
pub mod extension;

/// Signature
pub mod signature;


pub mod versions {
    pub const LIBERATO_IDENTITY_VERSION: &str = "LIBERATØv1.0";
    pub const LIBERATO_IDENTITY_VERSION_PEM_PUBLIC: &str = "LIBERATØv1.0-PUBLIC";
    pub const LIBERATO_IDENTITY_VERSION_PEM_PRIVATE: &str = "LIBERATØv1.0-PRIVATE";
    pub const LIBERATO_IDENTITY_VERSION_PEM_SIGNATURE: &str = "LIBERATØv1.0-SIGNATURE";
}

/// # LiberatoIdentityAPI
/// 
/// ## Description
/// 
/// The Liberato Identity API is a simple API interface for interacting with Liberato/Muscarine Identities.
pub struct LiberatoIdentityAPI;

impl GenerateLiberatoIdentity for LiberatoIdentityAPI {
    fn generate_liberato_identity_with_extension<T: AsRef<str>>(alg: Slug20Algorithm, ext: T) -> LiberatoIdentity {
        let x: OpenInternetCryptographyKeypair = OpenInternetCryptographyKeypair::generate_with_algorithm(alg.clone()).unwrap();

        let output: String = LiberatoIdentity::derive_liberato_address(x.into_public_key().into_standard_pem().unwrap().as_str(), Some(ext.as_ref()));
        let cipher_suite: CipherSuite = CipherSuite::new(alg.clone());

        let pk_output_address: String = LiberatoIdentity::derive_liberato_address_pk(x.into_public_key().into_standard_pem().unwrap());

        LiberatoIdentity {
            address: str192::from_str(&output).unwrap(),
            address_pk: str192::from_str(&pk_output_address).unwrap(),
            public_key: x.into_public_key().into_standard_pem().unwrap(),
            secret_key: x.into_secret_key().into_standard_pem().unwrap(),
            cipher_suite: cipher_suite,
            extension: ext.as_ref().to_string(),
            data_derived: None,
            nonce: 0,
        }
    }
    fn generate_liberato_identity(alg: Slug20Algorithm) -> LiberatoIdentity {
        return LiberatoIdentityAPI::generate_liberato_identity_with_extension(alg,versions::LIBERATO_IDENTITY_VERSION);
    }
}

/// # LiberatoIdentity
/// 
/// ## Description
/// 
/// The Liberato Identity type is a simple type for representing Liberato/Muscarine Identities.
/// 
/// ## Fields
/// 
/// - [X] Address
/// - [X] Address PK
/// - [X] Public Key
/// - [X] Secret Key
/// - [X] Cipher Suite
/// - [X] Extension
/// - [X] Data Derived
/// - [X] Nonce
/// 
/// ## Methods
/// 
/// - [X] Derive Liberato Address
/// 
/// ## Example
/// 
/// ```rust
/// use liberato_address_scheme::identity::LiberatoIdentity;
/// 
/// fn main() {
///     let x: LiberatoIdentity = LiberatoIdentityAPI::generate_liberato_identity(Slug20Algorithm::ShulginSigning);
/// }
/// ```
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
pub struct LiberatoIdentity {
    // Addressing Scheme
    pub address: str192, // BLAKE2B (48)
    pub address_pk: str192, // BLAKE2B (48) | Format: PEM-ENCODED
    
    pub public_key: String,
    pub secret_key: String,
    pub cipher_suite: CipherSuite,
    
    pub extension: String,
    pub data_derived: Option<String>,
    pub nonce: u64,
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
pub struct LiberatoIdentityPublic {
    pub address: str192,
    pub address_pk: str192,
    
    pub public_key: String,
    pub cipher_suite: CipherSuite,
    
    pub extension: String,
    pub data_derived: Option<String>,
    pub nonce: u64,
}

impl LiberatoIdentityPublic {
    pub fn from_identity(identity: LiberatoIdentity) -> LiberatoIdentityPublic {
        LiberatoIdentityPublic {
            address: identity.address,
            address_pk: identity.address_pk,
            public_key: identity.public_key,
            cipher_suite: identity.cipher_suite,
            extension: identity.extension,
            data_derived: identity.data_derived,
            nonce: identity.nonce,
        }
    }
    pub fn as_address(&self) -> String { 
        self.address.to_string() 
    }
    pub fn as_address_pk(&self) -> String {
        self.address_pk.to_string()
    }
    pub fn as_public_key(&self) -> String {
        self.public_key.clone()
    }
    pub fn as_cipher_suite(&self) -> CipherSuite {
        self.cipher_suite.clone()
    }
    pub fn as_extension(&self) -> String {
        self.extension.clone()
    }
    pub fn as_data_derived(&self) -> Option<String> {
        self.data_derived.clone()
    }
    pub fn as_nonce(&self) -> u64 {
        self.nonce
    }
    pub fn into_bincode(&self) -> Result<Vec<u8>, crate::errors::LiberatoAddressError> {
        let x = bincode::serialize(&self);
        if x.is_ok() {
            return Ok(x.unwrap())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn from_bincode(bytes: &[u8]) -> Result<LiberatoIdentityPublic, crate::errors::LiberatoAddressError> {
        let x = bincode::deserialize(bytes);
        if x.is_ok() {
            return Ok(x.unwrap())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn into_liberato_address_pem(&self) -> String {
        let x: Vec<u8> = self.into_bincode().unwrap();
        Pem::new(versions::LIBERATO_IDENTITY_VERSION_PEM_PUBLIC, x).to_string()
    }
    pub fn from_liberato_address_pem<T: AsRef<str>>(key_as_pem: T) -> Result<LiberatoIdentityPublic, crate::errors::LiberatoAddressError> {
        let x = Pem::from_str(key_as_pem.as_ref());
        if x.is_ok() {
            return LiberatoIdentityPublic::from_bincode(x.unwrap().contents())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    /// # Verify
    /// 
    /// Verify a message with a signature.
    pub fn verify<T: AsRef<[u8]>>(&self, message: T, signature: LiberatoSignature) -> bool {
        let x: OpenInternetCryptographyPublicKey = self.into_public_key();
        let is_valid = x.verify(message.as_ref(), &signature.signature);

        if is_valid.is_err() {
            return false
        }
        else {
            return is_valid.unwrap()
        }
    }

    pub fn into_public_key(&self) -> OpenInternetCryptographyPublicKey {
        OpenInternetCryptographyPublicKey::from_standard_pem_with_algorithm(self.public_key.as_str(), self.cipher_suite.algorithm()).unwrap()
    }
}

impl LiberatoIdentity {
    pub fn into_identity_public(&self) -> LiberatoIdentityPublic {
        LiberatoIdentityPublic {
            address: self.address,
            address_pk: self.address_pk,
            public_key: self.public_key.clone(),
            cipher_suite: self.cipher_suite.clone(),
            extension: self.extension.clone(),
            data_derived: self.data_derived.clone(),
            nonce: self.nonce,
        }
    }
    pub fn into_bincode(&self) -> Result<Vec<u8>, crate::errors::LiberatoAddressError> {
        let x = bincode::serialize(&self);

        if x.is_ok() {
            return Ok(x.unwrap())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn from_bincode(bytes: &[u8]) -> Result<LiberatoIdentity, crate::errors::LiberatoAddressError> {
        let x = bincode::deserialize(bytes);
        if x.is_ok() {
            return Ok(x.unwrap())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn into_liberato_address_pem(&self) -> String {
        let x: Vec<u8> = self.into_bincode().unwrap();
        Pem::new(versions::LIBERATO_IDENTITY_VERSION, x).to_string()
    }
    pub fn get_pem_label() -> String {
        String::from(versions::LIBERATO_IDENTITY_VERSION)
    }
    pub fn from_liberato_address_pem<T: AsRef<str>>(key_as_pem: T) -> Result<LiberatoIdentity, crate::errors::LiberatoAddressError> {
        let x: Result<Pem, pem::PemError> = Pem::from_str(key_as_pem.as_ref());

        if x.is_ok() {
            let y: Result<LiberatoIdentity, Box<bincode::ErrorKind>> = bincode::deserialize(x.unwrap().contents());
            if y.is_ok() {
                return Ok(y.unwrap())
            }
            else {
                return Err(crate::errors::LiberatoAddressError::Unknown)
            }
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn as_public(&self) -> LiberatoIdentityPublic {
        LiberatoIdentityPublic {
            address: self.address,
            address_pk: self.address_pk,
            public_key: self.public_key.clone(),
            cipher_suite: self.cipher_suite.clone(),
            extension: self.extension.clone(),
            data_derived: self.data_derived.clone(),
            nonce: self.nonce,
        }
    }
    pub fn as_address(&self) -> String { 
        self.address.to_string()
    }
    pub fn as_address_pk(&self) -> String {
        self.address_pk.to_string()
    }
    pub fn as_cipher_suite(&self) -> CipherSuite {
        self.cipher_suite.clone()
    }
    pub fn as_extension(&self) -> String {
        self.extension.clone()
    }
    pub fn as_data_derived(&self) -> Option<String> {
        self.data_derived.clone()
    }
    pub fn as_nonce(&self) -> u64 {
        self.nonce
    }
    pub fn sign<T: AsRef<[u8]>>(&self, message: T) -> Result<LiberatoSignature, crate::errors::LiberatoAddressError> {
        let x: OpenInternetCryptographySecretKey = self.into_private_key()?;
        let sig = x.sign(message.as_ref());
        if sig.is_ok() {
            return Ok(LiberatoSignature { signature: sig.unwrap() })
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn into_private_key(&self) -> Result<OpenInternetCryptographySecretKey, crate::errors::LiberatoAddressError> {
        let x = OpenInternetCryptographySecretKey::from_standard_pem_with_algorithm(self.secret_key.as_str(), self.cipher_suite.algorithm());
        if x.is_ok() {
            return Ok(x.unwrap())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn into_public_key(&self) -> Result<OpenInternetCryptographyPublicKey, crate::errors::LiberatoAddressError> {
        let x = OpenInternetCryptographyPublicKey::from_standard_pem_with_algorithm(self.public_key.as_str(), self.cipher_suite.algorithm());
        if x.is_ok() {
            return Ok(x.unwrap())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
}



impl DeriveLiberatoAddress for LiberatoIdentity {
    fn derive_liberato_address<T: AsRef<str>>(public_key: T, extension: Option<T>) -> String {
        let mut hasher = Blake2b::new(48);
        hasher.update(public_key.as_ref().as_bytes());
        if let Some(extension) = extension {
            hasher.update(extension.as_ref().as_bytes());
        } else {
            hasher.update("LIBERATØv1.0".as_bytes());
        }
        let output: blake2_rfc::blake2b::Blake2bResult = hasher.finalize();
        let mut address: String = String::new();
        for byte in output.as_bytes() {
            address.push_str(&format!("{:02x}", byte));
        }
        address
    }

    fn derive_liberato_address_pk<T: AsRef<str>>(public_key: T) -> String {
        let mut hasher = Blake2b::new(48);
        hasher.update(public_key.as_ref().as_bytes());
        let output: blake2_rfc::blake2b::Blake2bResult = hasher.finalize();
        let mut address: String = String::new();
        for byte in output.as_bytes() {
            address.push_str(&format!("{:02x}", byte));
        }
        address
    }
}
#[test]
fn generate_liberato_address_using_shulgin_signing() {
    let x: LiberatoIdentity = LiberatoIdentityAPI::generate_liberato_identity(Slug20Algorithm::ShulginSigning);
    println!("{}",serde_json::to_string_pretty(&x).unwrap());
}

#[test]
fn hash_and_go() {
    let x: LiberatoIdentity = LiberatoIdentityAPI::generate_liberato_identity(Slug20Algorithm::ShulginSigning);
    let y: LiberatoSignature = x.sign("message".as_bytes()).unwrap();
    let z: LiberatoIdentityPublic = x.into_identity_public();

    println!("{}",z.verify("message", y.clone()));

    println!("{}",y.into_liberato_address_pem());
    println!("{}",x.into_liberato_address_pem());
}