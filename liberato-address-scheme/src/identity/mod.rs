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

use crate::prelude::CipherSuite;

/// Cipher Suite
pub mod cipher_suite;

/// Extensions To Liberato Identity
pub mod extension;


pub mod versions {
    pub const LIBERATO_IDENTITY_VERSION: &str = "LIBERATØv1.0";
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
    pub public_key: String,
    pub cipher_suite: CipherSuite,
    pub extension: String,
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