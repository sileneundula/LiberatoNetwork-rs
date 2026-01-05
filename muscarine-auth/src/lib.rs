/// # MuscarineAuth API
/// 
/// ## Description
/// 
/// **Muscarine Auth** is a centralized/decentralized way of authenticating clients with other services or servers.
/// 
/// ## Features
/// 
/// - [ ] SecretKey
/// - [ ] DigitalSignatureOnRNG
/// - [ ] DigitalSignatureOnMuscarineVRF (0x20CB)
/// - [ ] DigitalSignatureAgainstDomain
/// - [ ] DigitalSignatureAgainstPublicDatabase (0x20CB)
pub struct MuscarineAuthAPI;

pub mod server;

pub mod client;



pub trait MuscarineAuthenticate {
    fn authenticate() -> bool;
}

pub enum MuscarineAuthMethods {
    SecretKey,
    
    DigitalSigning,
    
    DigitalSigningWithDomain,
    DigitalSigningWithP2P,

    DigitalSigningWithMuscarineVRFChain,
}