use std::str::FromStr;

use fixedstr::str128;
use openinternetcryptographykeys::prelude::essentials::Slug20Algorithm;
use serde::{Serialize,Deserialize};

pub struct CipherSuiteAPI;



#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
/// # CipherSuite
/// 
/// ## Description
/// 
/// The CipherSuite type is a simple type for representing cipher suites.
pub struct CipherSuite(pub str128);

impl CipherSuiteAPI {
    pub fn new(alg: Slug20Algorithm) -> CipherSuite {
        return CipherSuite::new(alg);
    }
}

impl CipherSuite {
    pub fn new(alg: Slug20Algorithm) -> CipherSuite {
        match alg {
            Slug20Algorithm::AbsolveSigning => return CipherSuite(str128::from_str("slug20_absolvesigning_ml_dsa_and_ed25519").unwrap()),
            Slug20Algorithm::EsphandSigning => return CipherSuite(str128::from_str("slug20_esphandsigning_falcon1024_and_ed25519").unwrap()),
            Slug20Algorithm::ShulginSigning => return CipherSuite(str128::from_str("slug20_shulginsigning_sphincs_and_ed25519").unwrap()),
            Slug20Algorithm::Ed25519 => return CipherSuite(str128::from_str("slug20_ed25519").unwrap()),
            Slug20Algorithm::Ed448 => return CipherSuite(str128::from_str("slug20_ed448").unwrap()),
            Slug20Algorithm::ECDSA => return CipherSuite(str128::from_str("slug20_ecdsa").unwrap()),
            _ => panic!("Not Standardized CipherSuite")
        }
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    pub fn algorithm(&self) -> Slug20Algorithm {
        match self.as_str() {
            "slug20_absolvesigning_ml_dsa_and_ed25519" => return Slug20Algorithm::AbsolveSigning,
            "slug20_esphandsigning_falcon1024_and_ed25519" => return Slug20Algorithm::EsphandSigning,
            "slug20_shulginsigning_sphincs_and_ed25519" => return Slug20Algorithm::ShulginSigning,
            "slug20_ed25519" => return Slug20Algorithm::Ed25519,
            "slug20_ed448" => return Slug20Algorithm::Ed448,
            "slug20_ecdsa" => return Slug20Algorithm::ECDSA,
            _ => panic!("Not Standardized CipherSuite")
        }
    }
}