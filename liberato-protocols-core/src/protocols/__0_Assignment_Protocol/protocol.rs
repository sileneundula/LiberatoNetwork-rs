//! # Assignment Protocol
//! 
//! ## Description
//! 
//! The Assignment Protocol is the protocol used to assign labels to nodes.

//use libp2p::swarm::StreamProtocol;

use libp2p::request_response::Codec;
use libp2p_core::upgrade::{InboundUpgrade, OutboundUpgrade, UpgradeInfo};

pub const LIBERATO_ASSIGNMENT_PROTOCOL: StreamProtocol = StreamProtocol::new("/liberato/AssignmentProtocol/1.0.0");


// MAX PACKET SIZE
pub const DEFAULT_MAX_PACKET_SIZE: usize = 16 * 1024;
/// The default timeout of outbound_substreams to be 10 (seconds).
const DEFAULT_SUBSTREAMS_TIMEOUT_S: Duration = Duration::from_secs(10);

pub struct AssignmentProtocol;

impl Codec for AssignmentProtocol {
    type Protocol = "/liberato/assignment-protocol/0.5.0";
    type Request = ;
    type Response = ;
    
}