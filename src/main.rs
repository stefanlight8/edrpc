mod game;
mod loadout;
mod state;

use {
    daito::{
        api::activity::{Activity, ActivityAssets},
        rpc::client::RpcClient,
    },
    std::{
        env, error::Error, os::unix::net::UnixStream, path::PathBuf, process::id, time::Duration,
    },
    tokio::time::sleep,
    tracing_subscriber::EnvFilter,
};

pub fn get_discord_ipc_pipe() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    let base = PathBuf::from(std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into()));

    #[cfg(target_os = "macos")]
    let base = PathBuf::from(std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".into()));

    for i in 0..10 {
        let path = base.join(format!("discord-ipc-{}", i));

        if UnixStream::connect(&path).is_ok() {
            return Some(path);
        }
    }

    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pipe = get_discord_ipc_pipe().expect("expected pipe");
    tracing::info!("using pipe: {}", pipe.display());

    let mut client =
        RpcClient::open(env::var("CLIENT_ID").expect("expected CLIENT_ID"), pipe).await?;
    client.handshake().await?;

    let activities = vec![
        Activity {
            name: "Elite Dangerous".to_string(),
            details: Some("In game".to_string()),
            assets: Some(ActivityAssets {
                large_image: Some("elite-dangerous".to_string()),
                large_text: Some("Elite Dangerous".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
        Activity {
            name: "Elite Dangerous".to_string(),
            details: Some("Supercruise".to_string()),
            state: Some("San Tu".to_string()),
            assets: Some(ActivityAssets {
                large_image: Some("ship".to_string()),
                large_text: Some("Fer-de-lance (Enma)".to_string()),
                small_image: Some("elite-dangerous-minimalistic".to_string()),
                small_text: Some("Elite Dangerous".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
        Activity {
            name: "Elite Dangerous".to_string(),
            details: Some("Docked".to_string()),
            state: Some("Chomsky Station".to_string()),
            assets: Some(ActivityAssets {
                large_image: Some("ship".to_string()),
                large_text: Some("Fer-de-lance (Enma)".to_string()),
                small_image: Some("elite-dangerous-minimalistic".to_string()),
                small_text: Some("Elite Dangerous".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
        Activity {
            name: "Elite Dangerous".to_string(),
            details: Some("In SRV".to_string()),
            state: Some("San Tu 1 a".to_string()),
            assets: Some(ActivityAssets {
                large_image: Some("srv".to_string()),
                large_text: Some("Scorpio".to_string()),
                small_image: Some("elite-dangerous-minimalistic".to_string()),
                small_text: Some("Elite Dangerous".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
        Activity {
            name: "Elite Dangerous".to_string(),
            details: Some("On foot".to_string()),
            state: Some("San Tu 1 a".to_string()),
            assets: Some(ActivityAssets {
                large_image: Some("helmet".to_string()),
                large_text: Some("Artemis".to_string()),
                small_image: Some("elite-dangerous-minimalistic".to_string()),
                small_text: Some("Elite Dangerous".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
    ];
    let mut position = 0;
    loop {
        client
            .set_activity(id(), activities.get(position).unwrap().clone())
            .await?;

        position += 1;
        if position == activities.len() {
            position = 0
        }

        sleep(Duration::from_secs(5)).await;
    }
}
