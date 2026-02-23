use std::path::Path;

use serde_json::json;
use tracing::info;

use crate::{
    discord::activity::Activity,
    pipe::errors::PipeError,
    rpc::{
        errors::{HandshakeError, RpcError, TransportError},
        op_code::OpCode,
        payload::{Payload, RpcCommand},
        transport::RpcTransport,
        utils::get_nonce,
    },
};

pub struct RpcClient {
    pub client_id: String,
    pub transport: RpcTransport,
}

impl RpcClient {
    pub async fn open(
        client_id: String,
        pipe_path: impl AsRef<Path>,
    ) -> Result<RpcClient, PipeError> {
        let transport = RpcTransport::open(pipe_path).await?;

        Ok(Self {
            client_id,
            transport,
        })
    }

    pub async fn handshake(&mut self) -> Result<(), HandshakeError> {
        self.transport
            .send(
                OpCode::Hello,
                &Payload::Handshake {
                    version: 1,
                    client_id: self.client_id.to_string(),
                },
            )
            .await?;

        let (_, payload): (_, Payload) = self.transport.receive().await?;
        match payload {
            Payload::Event { evt: Some(evt), .. } if evt != "READY" => {
                return Err(HandshakeError::ProtocolError {
                    expected: "READY",
                    got: evt,
                });
            }
            Payload::Error { code, message } => {
                return Err(HandshakeError::RpcError(RpcError { code, message }));
            }
            _ => (),
        }

        Ok(())
    }

    pub async fn set_activity(&mut self, activity: Activity) -> Result<(), TransportError> {
        self.transport
            .send(
                OpCode::Dispatch,
                &Payload::Event {
                    cmd: RpcCommand::SetActivity,
                    args: Some(json!({
                        "pid": std::process::id(),
                        "activity": activity,
                    })),
                    data: None,
                    evt: None,
                    nonce: Some(get_nonce()),
                },
            )
            .await?;

        Ok(())
    }
}
