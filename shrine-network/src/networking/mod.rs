//! # Networking Internals Using Libp2p
//! 
//! ## Features
//! 
//! - [X] Connection (Transport)
//!     - [ ] Use of Websockets
//!     - [X] Use of Noise Protocol
//!     - [X] Use of Yamux Multiplexing
//! - [X] Commands
//!     - [ ] Implemented
//!         - [ ] lsp
//!         - [ ] ...
//! - [X] Behaviour
//!     - [ ] Peer Discovery
//!         - [X] KAD
//!         - [ ] MDNS

pub mod internals;
pub mod bootstrap;
pub mod protocols;