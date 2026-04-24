use std::str::FromStr;

use openinternetcryptographykeys::prelude::essentials::OpenInternetCryptographySignature;
use crate::identity::versions::LIBERATO_IDENTITY_VERSION_PEM_SIGNATURE;
use pem::Pem;
use serde::{Serialize,Deserialize};

/// # Liberato Signature
/// 
/// The Liberato Signature type is a simple type for representing Liberato/Muscarine Signatures.
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
pub struct LiberatoSignature {
    pub signature: OpenInternetCryptographySignature,
}

/// # Liberato Signature With Message
/// 
/// The Liberato Signature With Message type is a simple type for representing Liberato/Muscarine Signatures.
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,PartialOrd,Hash)]
pub struct LiberatoSignatureWithMessage {
    pub signature: LiberatoSignature,
    pub message: Vec<u8>,
}

impl LiberatoSignature {
    pub fn into_bincode(&self) -> Result<Vec<u8>, crate::errors::LiberatoAddressError> {
        let x = bincode::serialize(&self.signature);
        if x.is_ok() {
            return Ok(x.unwrap())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn from_bincode(bytes: &[u8]) -> Result<LiberatoSignature, crate::errors::LiberatoAddressError> {
        let x = bincode::deserialize(bytes);
        if x.is_ok() {
            return Ok(LiberatoSignature { signature: x.unwrap() })
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn from_liberato_address_pem<T: AsRef<str>>(key_as_pem: T) -> Result<LiberatoSignature, crate::errors::LiberatoAddressError> {
        let x = Pem::from_str(key_as_pem.as_ref());
        if x.is_ok() {
            return LiberatoSignature::from_bincode(x.unwrap().contents())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    /// # Into Liberato Address PEM
    /// 
    /// Converts a LiberatoSignature to a Liberato Address PEM
    pub fn into_liberato_address_pem(&self) -> String {
        let x: Vec<u8> = self.into_bincode().unwrap();
        Pem::new(LIBERATO_IDENTITY_VERSION_PEM_SIGNATURE, x).to_string()
    }
    /// # To Signature
    /// 
    /// Converts a LiberatoSignature to an OpenInternetCryptographySignature
    pub fn to_signature(&self) -> OpenInternetCryptographySignature {
        self.signature.clone()
    }
    /// # From Signature
    /// 
    /// Converts an OpenInternetCryptographySignature to a LiberatoSignature
    pub fn from_signature(signature: OpenInternetCryptographySignature) -> LiberatoSignature {
        LiberatoSignature { signature: signature }
    }
}

impl LiberatoSignatureWithMessage {
    pub fn into_bincode_signature(&self) -> Result<Vec<u8>, crate::errors::LiberatoAddressError> {
        let x = bincode::serialize(&self.signature);
        if x.is_ok() {
            return Ok(x.unwrap())
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn from_liberato_address_pem_signature_and_message<T: AsRef<str>, U: AsRef<[u8]>>(key_as_pem: T, msg: U) -> Result<LiberatoSignatureWithMessage, crate::errors::LiberatoAddressError> {
        let x = Pem::from_str(key_as_pem.as_ref());
        if x.is_ok() {
            return Ok(LiberatoSignatureWithMessage {
                signature: LiberatoSignature::from_bincode(x.unwrap().contents()).unwrap(),
                message: msg.as_ref().to_vec()
            })
        }
        else {
            return Err(crate::errors::LiberatoAddressError::Unknown)
        }
    }
    pub fn into_liberato_address_pem_signature(&self) -> String {
        let x: Vec<u8> = self.into_bincode_signature().unwrap();
        Pem::new(LIBERATO_IDENTITY_VERSION_PEM_SIGNATURE, x).to_string()
    }
}