use async_trait::async_trait;
use futures::prelude::*;
use futures::{AsyncRead, AsyncWrite, AsyncWriteExt};
use libp2p::request_response::Codec;
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

// NOTE: yoinked from libp2p
// <https://github.com/libp2p/rust-libp2p/blob/ce9821154a3bde53e38e72c511acbacb721573ce/core/src/upgrade/transfer.rs?plain=1#L28>
/// Writes a message to the given socket with a length prefix appended to it. Also flushes the socket.
///
/// > **Note**: Prepends a variable-length prefix indicate the length of the message. This is
/// >           compatible with what [`read_length_prefixed`] expects.
/// # Errors
/// On weird input from socket
pub async fn write_length_prefixed(
    socket: &mut (impl AsyncWrite + Unpin),
    data: impl AsRef<[u8]>,
) -> Result<(), io::Error> {
    write_varint(socket, data.as_ref().len()).await?;
    socket.write_all(data.as_ref()).await?;
    socket.flush().await?;

    Ok(())
}

/// Writes a variable-length integer to the `socket`.
///
/// > **Note**: Does **NOT** flush the socket.
/// # Errors
/// On weird input from socket
pub async fn write_varint(
    socket: &mut (impl AsyncWrite + Unpin),
    len: usize,
) -> Result<(), io::Error> {
    let mut len_data = unsigned_varint::encode::usize_buffer();
    let encoded_len = unsigned_varint::encode::usize(len, &mut len_data).len();
    socket.write_all(&len_data[..encoded_len]).await?;

    Ok(())
}

/// Reads a variable-length integer from the `socket`.
///
/// As a special exception, if the `socket` is empty and EOFs right at the beginning, then we
/// return `Ok(0)`.
///
/// > **Note**: This function reads bytes one by one from the `socket`. It is therefore encouraged
/// >           to use some sort of buffering mechanism.
/// # Errors
/// On weird input from socket
pub async fn read_varint(socket: &mut (impl AsyncRead + Unpin)) -> Result<usize, io::Error> {
    let mut buffer = unsigned_varint::encode::usize_buffer();
    let mut buffer_len = 0;

    loop {
        match socket.read(&mut buffer[buffer_len..=buffer_len]).await? {
            0 => {
                // Reaching EOF before finishing to read the length is an error, unless the EOF is
                // at the very beginning of the substream, in which case we assume that the data is
                // empty.
                if buffer_len == 0 {
                    return Ok(0);
                }
                return Err(io::ErrorKind::UnexpectedEof.into());
            }
            n => debug_assert_eq!(n, 1),
        }

        buffer_len += 1;

        match unsigned_varint::decode::usize(&buffer[..buffer_len]) {
            Ok((len, _)) => return Ok(len),
            Err(unsigned_varint::decode::Error::Overflow) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "overflow in variable-length integer",
                ));
            }
            // TODO: why do we have a `__Nonexhaustive` variant in the error? I don't know how to process it
            // Err(unsigned_varint::decode::Error::Insufficient) => {}
            Err(_) => {}
        }
    }
}

/// Reads a length-prefixed message from the given socket.
///
/// The `max_size` parameter is the maximum size in bytes of the message that we accept. This is
/// necessary in order to avoid `DoS` attacks where the remote sends us a message of several
/// gigabytes.
///
/// > **Note**: Assumes that a variable-length prefix indicates the length of the message. This is
/// >           compatible with what [`write_length_prefixed`] does.
/// # Errors
/// On weird input from socket
pub async fn read_length_prefixed(
    socket: &mut (impl AsyncRead + Unpin),
    max_size: usize,
) -> io::Result<Vec<u8>> {
    let len = read_varint(socket).await?;
    if len > max_size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Received data size ({len} bytes) exceeds maximum ({max_size} bytes)"),
        ));
    }

    let mut buf = vec![0; len];
    socket.read_exact(&mut buf).await?;

    Ok(buf)
}

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
