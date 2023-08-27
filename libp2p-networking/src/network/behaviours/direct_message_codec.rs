use async_trait::async_trait;
use futures::{AsyncRead, AsyncWrite, AsyncWriteExt};
use libp2p::{
    core::upgrade::{read_length_prefixed, write_length_prefixed},
    request_response::Codec,
};
use serde::{Deserialize, Serialize};
use std::io;

/// Protocol for direct messages
#[derive(Debug, Clone)]
pub struct DirectMessageProtocol();
/// Codec for direct messages
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DirectMessageCodec();
/// Wrapper type describing a serialized direct message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectMessageRequest(pub Vec<u8>);
/// wrapper type describing the response to direct message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectMessageResponse(pub Vec<u8>);

/// Maximum size of a direct message
pub const MAX_MSG_SIZE_DM: usize = 100_000_000;

impl AsRef<str> for DirectMessageProtocol {
    fn as_ref(&self) -> &str {
        "/HotShot/request_response/1.0"
    }
}

#[async_trait]
impl Codec for DirectMessageCodec {
    type Protocol = DirectMessageProtocol;

    type Request = DirectMessageRequest;

    type Response = DirectMessageResponse;

    async fn read_request<T>(
        &mut self,
        _: &DirectMessageProtocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let msg = read_length_prefixed(io, MAX_MSG_SIZE_DM).await?;

        // NOTE we don't error here unless message is too big.
        // We'll wrap this in a networkbehaviour and get parsing messages there
        Ok(DirectMessageRequest(msg))
    }

    async fn read_response<T>(
        &mut self,
        _: &DirectMessageProtocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let msg = read_length_prefixed(io, MAX_MSG_SIZE_DM).await?;
        Ok(DirectMessageResponse(msg))
    }

    async fn write_request<T>(
        &mut self,
        _: &DirectMessageProtocol,
        io: &mut T,
        DirectMessageRequest(msg): DirectMessageRequest,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, msg).await?;
        io.close().await?;

        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &DirectMessageProtocol,
        io: &mut T,
        DirectMessageResponse(msg): DirectMessageResponse,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, msg).await?;
        io.close().await?;
        Ok(())
    }
}
