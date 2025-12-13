use std::io;
use std::iter;
use futures::future::BoxFuture;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use serde::{Deserialize, Serialize};

use libp2p::request_response::{
    ProtocolName, ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    Ping,
    Pong,
    Exec { cmd: String },
    Ack { msg: String },
}

#[derive(Clone)]
pub struct CommandProtocol();
impl ProtocolName for CommandProtocol {
    fn protocol_name(&self) -> &[u8] {
        b"/shrine/command/1"
    }
}

#[derive(Clone)]
pub struct CommandCodec;

impl RequestResponseCodec for CommandCodec {
    type Protocol = CommandProtocol;
    type Request = Command;
    type Response = Command;

    fn read_request<T>(&mut self, _: &Self::Protocol, io: &mut T) -> BoxFuture<Result<Self::Request, io::Error>>
    where
        T: AsyncRead + Unpin + Send + 'static,
    {
        Box::pin(async move {
            // length-prefixed (u32 BE) JSON payload
            let mut len_buf = [0u8; 4];
            io.read_exact(&mut len_buf).await?;
            let len = u32::from_be_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            io.read_exact(&mut buf).await?;
            serde_json::from_slice(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
    }

    fn read_response<T>(&mut self, _: &Self::Protocol, io: &mut T) -> BoxFuture<Result<Self::Response, io::Error>>
    where
        T: AsyncRead + Unpin + Send + 'static,
    {
        // same format as request
        self.read_request(&CommandProtocol(), io)
    }

    fn write_request<T>(&mut self, _: &Self::Protocol, io: &mut T, request: Self::Request) -> BoxFuture<Result<(), io::Error>>
    where
        T: AsyncWrite + Unpin + Send + 'static,
    {
        Box::pin(async move {
            let bytes = serde_json::to_vec(&request).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let len = (bytes.len() as u32).to_be_bytes();
            io.write_all(&len).await?;
            io.write_all(&bytes).await?;
            io.close().await?;
            Ok(())
        })
    }

    fn write_response<T>(&mut self, _: &Self::Protocol, io: &mut T, response: Self::Response) -> BoxFuture<Result<(), io::Error>>
    where
        T: AsyncWrite + Unpin + Send + 'static,
    {
        // same format as request
        self.write_request(&CommandProtocol(), io, response)
    }
}

/// Create a RequestResponse behaviour ready to exchange Command messages.
pub fn new_command_behaviour() -> RequestResponse<CommandCodec> {
    let protocols = iter::once((CommandProtocol(), ProtocolSupport::Full));
    RequestResponse::new(CommandCodec, protocols, RequestResponseConfig::default())
}