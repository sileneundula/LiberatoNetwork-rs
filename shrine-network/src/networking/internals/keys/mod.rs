use libp2p::identity::*;

pub struct ShrineKeys;

impl ShrineKeys {
    pub fn generate_ed25519() -> Keypair {
        return Keypair::generate_ed25519()
    }
    pub fn generate_ecdsa() -> Keypair {
        return Keypair::generate_ecdsa()
    }
    pub fn generate_secp256k1() -> Keypair {
        return Keypair::generate_secp256k1()
    }
}