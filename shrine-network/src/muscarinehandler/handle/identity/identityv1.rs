use std::str::FromStr;

use fixedstr::str256;
use fixedstr::tstr;
use crate::muscarinehandler::handle::traits::MuscarineIdentity;
use libslug::prelude::SlugDigest;
use libslug::slugcrypt::internals::signature::shulginsigning;
use libslug::slugcrypt::internals::signature::esphand_signature;
use libslug::slugcrypt::internals::signature::utils::signing_csprng;

#[derive(Clone,Copy,Hash,PartialEq,PartialOrd,Debug)]
pub struct MuscarineIdentityV1 {
    public_key: tstr<4096>, // TODO: Make 4000 bytes long
    cipher_suite: str256,
}

#[derive(Clone,Copy,Hash,PartialEq,PartialOrd,Debug)]
pub struct MuscarineIdentityV1WithSecretKey {
    public_key: tstr<4096>,
    secret_key: tstr<4096>,
    cipher_suite: str256,
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Hash)]
pub enum AddressScheme {
    X59FMT,
    X59FMT_HEX,
    X59FMT_BASE58,
    X59FMT_BASE32,
}

pub enum CipherSuites {
    ShulginSigning, // SPHINCS+ (SHAKE256) & ED25519
    EsphandSigning, // FALCON1024 + ED25519
    AbsolveSigning, // ML-DSA3 + ED25519
}


// Add Cipher Suite Checksum

impl CipherSuites {
    pub fn cipher_suite_id(&self) -> str256 {
        let x = self.clone();
        match x {
            CipherSuites::ShulginSigning => return str256::from_str("slug20_shulginsigning_sphincs_and_ed25519").unwrap(),
            CipherSuites::EsphandSigning => return str256::from_str("slug20_esphandsigning_falcon1024_and_ed25519").unwrap(),
            CipherSuites::AbsolveSigning => return str256::from_str("slug20_absolvesigning_ml_dsa_and_ed25519").unwrap(),
        }
    }
}


impl MuscarineIdentity for MuscarineIdentityV1 {
    /// # Address Scheme
    /// 
    /// `X59FMT/Hex`: BLAKE2B(48, tstr<4096>.as_bytes()) | Format: Hex
    fn address(&self, address_scheme: AddressScheme) -> str256 {
        let address = self.public_key;
        if address_scheme == AddressScheme::X59FMT || address_scheme == AddressScheme::X59FMT_HEX {
            let output = SlugDigest::blake2b(48, &address.as_bytes());
            let output = output.digest();
            assert_eq!(output.as_bytes().len(),96);
            return str256::from_str(output).unwrap()
        }
        else {
            return str256::new()
        }
    }
    fn generate(identity: CipherSuites) {
        match identity {
            CipherSuites::ShulginSigning => {
                let x = shulginsigning::ShulginKeypair::generate();
            }
            CipherSuites::AbsolveSigning => {

            }
            CipherSuites::EsphandSigning => {

            }
        }
    }
}