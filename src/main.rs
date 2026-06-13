mod game;
mod journal;
mod loadout;
mod location;
mod message;
mod rpc;
mod session;
mod state;
mod utils;

use {
    crate::{journal::JournalWatcher, message::Message, rpc::Rpc, utils::get_discord_ipc_pipe},
    std::{env, error::Error},
    tokio::{signal::ctrl_c, sync::mpsc},
    tracing_subscriber::EnvFilter,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let (tx, mut rx) = mpsc::channel(32);
    let journal_watcher = tokio::spawn(async move {
        let mut watcher = JournalWatcher::new(tx.clone());
        watcher
            .run(env::var("JOURNALS_PATH").unwrap().into())
            .await
            .unwrap_err();
    });

    let mut rpc = Rpc::new(env::var("CLIENT_ID")?, get_discord_ipc_pipe()).await?;
    rpc.start().await?;
    while let Some(message) = rx.recv().await {
        rpc.update(message).await?;
    }

    ctrl_c().await?;
    journal_watcher.await?;

    Ok(())
}
