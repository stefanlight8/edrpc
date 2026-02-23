use std::path::Path;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(unix)]
use crate::pipe::errors::PipeError;

#[cfg(unix)]
pub struct PipeTransport {
    stream: tokio::net::UnixStream,
}

#[cfg(unix)]
impl PipeTransport {
    pub async fn open(path: impl AsRef<Path>) -> Result<Self, PipeError> {
        let stream = tokio::net::UnixStream::connect(path)
            .await
            .map_err(|e| PipeError::ConnectionFailed(e))?;

        Ok(Self { stream })
    }
}

#[cfg(windows)]
pub struct PipeTransport {
    stream: tokio::net::windows::named_pipe::NamedPipeClient,
}

#[cfg(windows)]
impl PipeTransport {
    pub async fn open(url: impl AsRef<Path>) -> Result<Self, PipeError> {
        todo!()
    }
}

impl PipeTransport {
    pub async fn write_all(&mut self, buf: &[u8]) -> Result<(), PipeError> {
        Ok(self
            .stream
            .write_all(buf)
            .await
            .map_err(|e| PipeError::TransportError(e))?)
    }

    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), PipeError> {
        self.stream
            .read_exact(buf)
            .await
            .map_err(|e| PipeError::TransportError(e))?;

        Ok(())
    }
}
