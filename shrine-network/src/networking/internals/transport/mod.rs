use libp2p::{Transport, identity};
use libp2p::core::transport;
use libp2p::PeerId;
use libp2p::core::muxing;
use libp2p::tcp;
use libp2p::noise;
use libp2p::core;
use libp2p::yamux;
use libp2p::websocket;

pub struct ShrineTransport(pub transport::Boxed<(PeerId,muxing::StreamMuxerBox)>);

impl ShrineTransport {
    pub fn new(keypair: identity::Keypair) -> Self {
        let transport: transport::Boxed<(PeerId, muxing::StreamMuxerBox)> = create_secure_transport_tcp(&keypair);

        return Self(transport)
    }
}

fn create_secure_transport_tcp(keypair: &identity::Keypair) -> transport::Boxed<(PeerId, muxing::StreamMuxerBox)> {
    let auth_config: noise::Config = noise::Config::new(&keypair).unwrap();
    let transport: transport::Boxed<(PeerId, muxing::StreamMuxerBox)> = tcp::tokio::Transport::default()
        .upgrade(core::upgrade::Version::V1)  // Use protocol upgrade version 1
        .authenticate(auth_config)    // Add Noise encryption
        .multiplex(yamux::Config::default()) // Multiplexing
        .boxed();

    return transport
}

fn create_secure_transport_wss(keypair: identity::Keypair) -> core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)> {
    // Noise Config
    let auth_config: noise::Config = noise::Config::new(&keypair).unwrap();

    // WebSocket Secure Protocol
    let transport: transport::Boxed<(PeerId, muxing::StreamMuxerBox)> = websocket::WsConfig::new(tcp::tokio::Transport::default())
        .upgrade(core::upgrade::Version::V1)
        .authenticate(auth_config)
        .multiplex(yamux::Config::default())
        .boxed();

    return transport
}

fn create_secure_transport_quic(keypair: identity::Keypair) -> core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)> {
    let auth_config = noise::Config::new(&keypair).unwrap();

}