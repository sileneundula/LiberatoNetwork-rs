//! # A Randomness Beacon
//! 
//! A Block Lattice (DAG) consisting of different seeds.

use std::error::Error;
use std::str::FromStr;

use fixedstr::str64;
use fixedstr::str128;
use fixedstr::str256;

use libslug::slugcrypt::internals::digest::blake3::Blake3Hasher;
use libslug::slugcrypt::internals::signature::schnorr;
use libslug::slugcrypt::internals::signature::schnorr::SchnorrSecretKey;
use libslug::prelude::SlugDigest;

// Encryption
use libslug::slugcrypt::internals::encryption::ecies::{ECIESDecrypt,ECIESEncrypt,ECPublicKey,ECSecretKey};

pub struct Block {
    csprng: str128,
    prev_block_hash: str64,
    signature: str256,
}

pub struct AccountHolder {
    addr: str256,
    
    public_key: str256,
    encryption_addr: str256,
    
    seed: str256,

    public_seed_hashed: str256,
    public_seed_preimage: str256,
}

impl AccountHolder {
    /// # Generate New Account Holder (MuscarineVRF)
    /// 
    /// Uses Schnorr Secret Key and Public Key to create a Verifiable Random Function given the public key that is provable.
    /// 
    /// ## Features
    /// 
    /// - [X] Schnorr VRF
    ///     - [ ] Implemented
    /// - [X] Seeding
    ///     - [X] Secret Seed (BLAKE2B(40))
    ///     - [X] Public Seed (BLAKE3(32))
    ///     - [X] Public Seed Preimage (as long as 256 bytes)
    /// - [X] Addr
    ///     - [X] BLAKE3(`SchnorrPK`)
    /// - [X] Encryption
    ///     - [X] ECIES-ED25519-silene
    ///     - [ ] Kyber768
    pub fn generate<T: AsRef<[u8]>, S: AsRef<str>>(secret_seed: T, public_seed: S, context: S) -> Result<(Self,SchnorrSecretKey),Errors> {
        let key = Self::generate_key();
        let pk = key.public_key();

        let output_pk = match pk {
            Ok(v) => v,
            Err(_) => return Err(Errors::SchnorrKeyError)
        };

        let output_base58 = output_pk.to_base58_string();

        let pkh: str256 = str256::from_str(&output_base58).unwrap();

        let hash_output = SlugDigest::blake3(&output_base58.as_bytes()).digest();
        let addr = str256::from_str(hash_output).unwrap();

        let seed_as_hashed = str256::from_str(SlugDigest::blake2b(40, secret_seed.as_ref()).digest()).unwrap();

        let public_seed = str256::from_str(SlugDigest::blake3(public_seed.as_ref().as_bytes()).digest()).unwrap();

        if public_seed.len() > 256 {
            return Err(Errors::SchnorrKeyError)
        }

        let public_seed_preimage = str256::from_str(public_seed.as_ref()).unwrap();

        let encryption_key = Self::generate_encryption_key();
        let encryption_public_key = encryption_key.public_key();

        let encryption_pk_hex = encryption_public_key.to_hex_string().unwrap();

        let encryption_pk_in_hex_output = str256::from_str(&encryption_pk_hex).unwrap();




        Ok(
            (Self {
            addr: addr,
            public_key: pkh,

            encryption_addr: encryption_pk_in_hex_output,

            // Secret Seed
            seed: seed_as_hashed,

            // Public Seed
            public_seed_hashed: public_seed,
            public_seed_preimage: public_seed_preimage,
            },
        key))

        

    }
    pub fn generate_key() -> schnorr::SchnorrSecretKey {
        return schnorr::SchnorrSecretKey::generate()
    }
    pub fn generate_encryption_key() -> ECSecretKey {
        return ECSecretKey::generate()
    }
}

#[test]
fn test() {
    schnorr::SchnorrSecretKey::generate()
}

pub enum Errors {
    SchnorrKeyError,
}