//! # Errors
//! 
//! 0x00|0: Unassigned Error
//! 0x01|1: KAD: Failed To Remove Peer From Peer ID Table Due To Peer Not Being Found In Record. 
//! 
//! 
//! 
//! 

use thiserror::Error;



#[derive(Error,Debug)]
pub enum NetworkingStackErrors {
    #[error("The Networking Error {0} has occured, this is in reference to the problem: {1} | Reason: {2}, Solution: {3}, Debug Info: {4}")]
    Networking(u16,String,String,String,String),

    #[error("Unknown Error, Information: {0}")]
    UnknownError(String),
}