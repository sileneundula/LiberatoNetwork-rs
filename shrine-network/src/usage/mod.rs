/// # Shrine Network API
/// 
/// Shrine Network Utility
pub struct ShrineNetworkAPI;

impl ShrineNetworkAPI {
    pub fn connect(connection: ShrineConnectionConfig) {
        
    }
}

pub enum ShrineConnectionStatus {
    Connected,
    Disconnected,
}

pub enum ShrineConnectionConfig {
    Bootstrap,
    Normal,
    Strict,
    Custom(u16),
}