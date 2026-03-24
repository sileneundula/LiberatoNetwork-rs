use fixedstr::str256;
use crate::muscarinehandler::handle::identity::identityv1::{AddressScheme, CipherSuites};

pub trait MuscarineEventInfo {
    fn event_id(&self) -> u64;
    fn event_protocol(&self) -> &str;
    fn event_checksum(&self) -> &str;
}

pub trait MuscarineIdentity {
    fn address(&self, address_scheme: AddressScheme) -> str256;
    fn generate(identity: CipherSuites) -> Self;
}

