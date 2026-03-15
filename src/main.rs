use {
    crate::{
        discord::rpc::{client::RpcClient, utils::get_discord_ipc_pipe},
        message::Message,
        presence::presence,
        watchdog::watchdog,
    },
    anyhow::{Context, Result},
    std::path::PathBuf,
    tokio::{select, sync::mpsc},
    tracing_subscriber::EnvFilter,
};

mod discord;
mod elite;
mod loadout;
mod message;
mod pipe;
mod presence;
mod state;
mod utils;
mod watchdog;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let (message_tx, message_rx) = mpsc::channel::<Message>(256);
    let journals_path = PathBuf::from(dotenvy::var("JOURNALS_PATH")?);
    let rpc_client = RpcClient::open(dotenvy::var("CLIENT_ID")?, get_discord_ipc_pipe())
        .await
        .with_context(|| "failed to open rpc client")?;

    select! {
        _ = watchdog(journals_path, message_tx) => {},
        _ = presence(rpc_client, message_rx) => {},
        _ = tokio::signal::ctrl_c() => {}
    }

    Ok(())
}
