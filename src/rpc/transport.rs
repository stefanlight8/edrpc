use std::path::Path;
use tracing::trace;

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    pipe::{errors::PipeError, transport::PipeTransport},
    rpc::{errors::TransportError, op_code::OpCode},
};

pub struct RpcTransport {
    pipe: PipeTransport,
    buffer: Vec<u8>,
}

impl RpcTransport {
    pub async fn open(pipe_path: impl AsRef<Path>) -> Result<Self, PipeError> {
        let pipe = PipeTransport::open(pipe_path).await?;

        Ok(Self {
            pipe,
            buffer: Vec::new(),
        })
    }

    pub async fn send<T: Serialize>(
        &mut self,
        op: OpCode,
        value: &T,
    ) -> Result<(), TransportError> {
        self.buffer.clear();

        let payload = serde_json::to_vec(value)?;

        self.buffer.extend_from_slice(&op.to_u32().to_le_bytes());
        self.buffer
            .extend_from_slice(&(payload.len() as u32).to_le_bytes());
        self.buffer.extend_from_slice(&payload);

        tracing::trace!("sending {:?}", serde_json::to_string(value));
        self.pipe.write_all(&self.buffer).await?;

        Ok(())
    }

    pub async fn receive<T: DeserializeOwned>(&mut self) -> Result<(OpCode, T), TransportError> {
        let mut header = [0u8; 8];

        self.pipe.read_exact(&mut header).await?;

        let op = u32::from_le_bytes(
            header[0..4]
                .try_into()
                .map_err(|e| TransportError::ParseError(Box::new(e)))?,
        );
        let len = u32::from_le_bytes(
            header[4..8]
                .try_into()
                .map_err(|e| TransportError::ParseError(Box::new(e)))?,
        ) as usize;

        self.buffer.resize(len, 0);
        self.pipe.read_exact(&mut self.buffer).await?;

        trace!(
            "received: op={:?} len={} payload={:?}",
            op,
            len,
            std::str::from_utf8(&self.buffer).unwrap_or("<non-utf8>")
        );

        let value = serde_json::from_slice(&self.buffer)?;
        let op = OpCode::from(op);

        Ok((op, value))
    }
}
