use fixedstr::{str64, str128, str256};

/// # X59 Certificate
/// 
/// ## Features
/// 
/// - Public Key
/// - 
pub struct X59Certificate {
    version: u8,

    pkh: str256,
    pkh_encrypt: Option<str256>,

    signature_algorithm: str64,
    encrypt_algorithm: str64,

    nonce: u64, // for spam protection and challenge
}

pub struct X60Certificate {
    
}

pub struct X61Certificate {

}