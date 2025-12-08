//! # libnamespace-pivot: A Domain System Based On Inputs
//! 
//! 
//! 
//! A Pivot is an 8-12 byte domain that is easily readable
//! 
//! ## Formats:
//! - Hexadecimal
//! - Base32
//! - Base58
//! 
//! ## Used For:
//! - Multiple Projects
//! 
//! 
//! ## Extension Codes
//! 
//! Have Extension Codes for Pivots/Namespaces that are used for extending functionality and must be rebuilt.

use slugencode::SlugEncodingUsage;

pub trait InputTypes {

}



pub struct InputBytes(Vec<u8>);
pub struct InputString(String);

/// # ExtensionCodes
/// 
/// Contains the:
/// 
/// - ID (ID Used and Service ID)
/// - Params
pub struct ExtensionCodes {
    // One Value
    id: ExtensionID,
    params: Vec<ExtensionParameters>,
}

pub struct ExtensionID {
    common_name: String,
    labels: Option<[String;10]>,
    assigned_name_route: Option<String>,

    ext_id: u64,
    service_id: u64,
}

pub struct ExtensionParameters {
    param: String,
    flag: u16,
}

/// # NamespaceInput
/// 
/// 
pub struct NamespaceInput<T: InputTypes>(T);


impl InputTypes for InputString {

}

impl InputTypes for InputBytes {

}
