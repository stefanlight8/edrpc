use {
    crate::{
        discord::activity::{Activity, ActivityTimestamps},
        discord::rpc::client::RpcClient,
        loadout::Loadout,
        rpc::Event,
        state::GameState,
    },
    anyhow::Result,
    std::sync::Arc,
    tokio::{
        select,
        sync::{Notify, mpsc::Receiver},
    },
};

pub struct GamePresence {
    event_rx: Receiver<Event>,
    rpc_client: RpcClient,
}

impl GamePresence {
    pub fn new(event_rx: Receiver<Event>, rpc_client: RpcClient) -> Self {
        Self {
            event_rx,
            rpc_client,
        }
    }

    pub async fn start(&mut self, shutdown: Arc<Notify>) -> Result<()> {
        self.rpc_client.handshake().await?;
        self.update_presence(shutdown).await;

        Ok(())
    }

    async fn update_presence(&mut self, shutdown: Arc<tokio::sync::Notify>) {
        let mut state: Option<String> = None;
        let mut details: Option<String> = None;
        let mut created_at: Option<i64> = None;

        loop {
            let event = select! {
                event = self.event_rx.recv() => event,
                _ = shutdown.notified() => {
                    tracing::info!("shutdown received, stopping update_presence");
                    break;
                }
            };

            match event {
                Some(Event::GameStateUpdate(state)) => match state {
                    GameState::Idle => details = Some("Idle".to_string()),
                    GameState::Docked(station) => details = Some(format!("Docked in {}", station)),
                    GameState::Location(location) => details = Some(location),
                    GameState::Supercruise(Some(system)) => {
                        details = Some(format!("Supercruise in {}", system));
                    }
                    GameState::Supercruise(None) => details = Some("Supercruise".to_string()),
                    GameState::Approaching(location) => {
                        details = Some(format!("Approaching {}", location))
                    }
                    GameState::Dead => details = Some("Dead".to_string()),
                    GameState::Landed(Some(body)) => details = Some(format!("Landed on {}", body)),
                    GameState::Landed(None) => details = Some("Landed".to_string()),
                    GameState::OnCarrier => details = Some("On carrier".to_string()),
                    GameState::JumpingTo(system) => {
                        details = Some(format!("Jumping to {}", system))
                    }
                },
                Some(Event::LoadoutUpdate(loadout)) => match loadout {
                    Loadout::OnFoot => state = Some("On foot".to_string()),
                    Loadout::Ship {
                        ship_name, ship_id, ..
                    } => state = Some(format!("{} ({})", ship_name, ship_id)),
                    Loadout::Srv => state = Some("On SRV".to_string()),
                    _ => state = None,
                },
                Some(Event::SessionUpdate {
                    created_at: datetime,
                }) => {
                    let timestamp = datetime.timestamp();

                    created_at = Some(timestamp);
                }
                None => {
                    tracing::warn!("event channel closed");
                    break;
                }
            };

            let _ = self
                .rpc_client
                .set_activity(Activity {
                    name: "Elite Dangerous".to_string(),
                    state: state.clone(),
                    details: details.clone(),
                    timestamps: Some(ActivityTimestamps {
                        start: created_at.clone(),
                        end: None,
                    }),
                    ..Default::default()
                })
                .await
                .inspect_err(|err| tracing::warn!("failed to update activity: {}", err));
        }
    }
}
