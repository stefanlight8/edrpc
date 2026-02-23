use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcCommand {
    Dispatch,
    SetActivity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Payload {
    Handshake {
        #[serde(alias = "v")]
        version: u32,
        client_id: String,
    },
    Event {
        cmd: RpcCommand,
        data: Option<Value>,
        args: Option<Value>,
        evt: Option<String>,
        nonce: Option<String>,
    },
    Error {
        code: usize,
        message: Option<String>,
    },
}
