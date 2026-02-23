use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use crate::pipe::errors::PipeError;

#[derive(Debug)]
pub struct RpcError {
    pub code: usize,
    pub message: Option<String>,
}

impl Error for RpcError {}

impl Display for RpcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} (code: {})",
            self.message.as_deref().unwrap_or("Unknown"),
            self.code
        )
    }
}

#[derive(Debug)]
pub enum HandshakeError {
    ProtocolError { expected: &'static str, got: String },
    RpcError(RpcError),
    TransportError(TransportError),
}

impl Error for HandshakeError {}

impl Display for HandshakeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            HandshakeError::ProtocolError { expected, got } => {
                write!(f, "Unexpected value: {}, expected: {}", got, expected)
            }
            HandshakeError::RpcError(e) => write!(f, "{}", e),
            HandshakeError::TransportError(e) => write!(f, "{}", e),
        }
    }
}

impl From<RpcError> for HandshakeError {
    fn from(e: RpcError) -> Self {
        HandshakeError::RpcError(e)
    }
}

impl From<TransportError> for HandshakeError {
    fn from(e: TransportError) -> Self {
        HandshakeError::TransportError(e)
    }
}

#[derive(Debug)]
pub enum TransportError {
    ParseError(Box<dyn Error + Send + Sync>),
    PipeError(PipeError),
}

impl Error for TransportError {}

impl Display for TransportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TransportError::ParseError(e) => write!(f, "Parse error: {}", e),
            TransportError::PipeError(e) => write!(f, "{}", e),
        }
    }
}

impl From<serde_json::Error> for TransportError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseError(Box::new(e))
    }
}

impl From<PipeError> for TransportError {
    fn from(e: PipeError) -> Self {
        Self::PipeError(e)
    }
}
