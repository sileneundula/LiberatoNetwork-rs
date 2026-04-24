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

/// # LiberatoIdentityAPI
/// 
/// ## Description
/// 
/// The Liberato Identity API is a simple API interface for interacting with Liberato/Muscarine Identities.
pub struct LiberatoIdentityAPI;

impl GenerateLiberatoIdentity for LiberatoIdentityAPI {
    fn generate_liberato_identity<T: AsRef<str>>(alg: Slug20Algorithm, ext: Option<T>) -> LiberatoIdentity {
        let x: OpenInternetCryptographyKeypair = OpenInternetCryptographyKeypair::generate_with_algorithm(alg).unwrap();

        let extension: Option<String> = match ext {
            Some(v) => Some(v.as_ref().to_string()),
            None => None,
        };

        let output: String = LiberatoIdentity::derive_liberato_address(x.into_public_key().into_standard_pem().unwrap().as_str(), None);
        let cipher_suite: CipherSuite = CipherSuite::new(alg);

        LiberatoIdentity {
            address: str192::from_str(&output).unwrap(),
            public_key: x.into_public_key().into_standard_pem().unwrap(),
            secret_key: Some(x.into_secret_key().into_standard_pem().unwrap()),
            cipher_suite: cipher_suite,
            extension: ext.unwrap_or(String::from("LIBERATO")),
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
pub struct LiberatoIdentity {
    pub address: str192, // BLAKE2B (48)
    pub public_key: String,
    pub secret_key: String,
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
            hasher.update("null".as_bytes());
        }
        let output: blake2_rfc::blake2b::Blake2bResult = hasher.finalize();
        let mut address: String = String::new();
        for byte in output.as_bytes() {
            address.push_str(&format!("{:02x}", byte));
        }
        address
    }
}