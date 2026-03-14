use {
    crate::{
        discord::rpc::{client::RpcClient, utils::get_discord_ipc_pipe},
        presence::GamePresence,
        rpc::Event,
        state_manager::GameStateManager,
    },
    anyhow::{Context, Result},
    std::{path::PathBuf, str::FromStr, sync::Arc},
    tokio::sync::mpsc,
};

mod discord;
mod elite;
mod loadout;
mod pipe;
mod presence;
mod rpc;
mod state;
mod state_manager;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let (event_tx, event_rx) = mpsc::channel::<Event>(256);
    let rpc_client = RpcClient::open(String::from("1473967338121592842"), get_discord_ipc_pipe())
        .await
        .with_context(|| "failed to open rpc client")?;

    let shutdown = Arc::new(tokio::sync::Notify::new());

    let presence = GamePresence::new(event_rx, rpc_client);
    let presence_task = tokio::spawn({
        let mut presence = presence;
        let shutdown = shutdown.clone();
        async move { presence.start(shutdown).await }
    });

    let state_manager = GameStateManager::new(
        event_tx,
        PathBuf::from_str(
            "/Users/stefanlight/Library/Application Support/Crossover/Bottles/Elite Dangerous/drive_c/users/crossover/Saved Games/Frontier Developments/Elite Dangerous",
        )?,
    );
    let state_manager_task = tokio::spawn({
        let state_manager = state_manager;
        let shutdown = shutdown.clone();
        async move { state_manager.start(shutdown).await }
    });

    tokio::signal::ctrl_c().await.unwrap();
    shutdown.notify_waiters();

    let _ = presence_task.await;
    let _ = state_manager_task.await;

    Ok(())
}
