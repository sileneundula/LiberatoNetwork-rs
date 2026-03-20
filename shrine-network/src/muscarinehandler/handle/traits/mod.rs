pub trait MuscarineEventInfo {
    fn event_id(&self) -> u64;
    fn event_protocol(&self) -> &str;
    fn event_checksum(&self) -> &str;
}