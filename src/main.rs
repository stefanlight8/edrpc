use anyhow::Result;

mod discord;
mod elite;
mod loadout;
mod pipe;
mod presence;
mod rpc;
mod state;
mod state_manager;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    Ok(())
}
