use {
    crate::{
        discord::{
            activity::{Activity, ActivityType},
            rpc::client::RpcClient,
        },
        loadout::Loadout,
        message::Message,
        state::GameState,
    },
    anyhow::{Result, anyhow},
    std::convert::Infallible,
    tokio::sync::mpsc::Receiver,
};

pub async fn presence(
    mut rpc_client: RpcClient,
    mut message_rx: Receiver<Message>,
) -> Result<Infallible> {
    rpc_client.handshake().await?;

    let mut session = Session::default();
    loop {
        let Some(message) = message_rx.recv().await else {
            return Err(anyhow!("message channel closed"));
            // FIXME: there's should be break, but Rust forces to return Ok(with something)
            // but with Infallible we can't return nothing, so we forced to return Err
            // (failed successfully to gracefully shutdown)
        };

        tracing::trace!("received message: {:?}", message);

        match message {
            Message::Session { created_at } => {}
            Message::Update { state, loadout } => session.patch(state, loadout),
        }

        let activity = Activity {
            name: "Elite Dangerous".into(),
            activity_type: ActivityType::Playing,
            state: Some(session.state.to_string()),
            details: Some(session.loadout.to_string()),
            ..Default::default()
        };

        rpc_client.set_activity(activity).await?;
    }
}

struct Session {
    state: GameState,
    loadout: Loadout,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            state: GameState::Idle,
            loadout: Loadout::Unknown,
        }
    }
}

impl Session {
    fn patch(&mut self, state: Option<GameState>, loadout: Option<Loadout>) {
        if let Some(state) = state {
            self.state = state
        }

        if let Some(loadout) = loadout {
            self.loadout = loadout
        }
    }
}

impl ToString for GameState {
    fn to_string(&self) -> String {
        match self {
            GameState::Idle => "Idle".to_string(),
            GameState::Dead => "Dead".to_string(),
            GameState::Approaching(location) => format!("Approaching {}", location),
            GameState::Docked(station_name) => format!("Docked in {}", station_name),
            GameState::JumpingTo(star_system) => format!("Jumping to {}", star_system),
            GameState::Supercruise(Some(star_system)) => format!("Supercruise in {}", star_system),
            GameState::Supercruise(..) => "Supercruise".to_string(),
            GameState::Landed(Some(body)) => format!("Landed on {}", body),
            GameState::Landed(..) => "Landed".to_string(),
            GameState::Location(location) => location.clone(),
            GameState::OnCarrier => "On fleet carrier".to_string(),
        }
    }
}

impl ToString for Loadout {
    fn to_string(&self) -> String {
        match self {
            Loadout::Ship {
                ship_type,
                ship_name,
                ship_id,
            } => format!("{} ({}, {})", ship_type, ship_name, ship_id),
            Loadout::Srv => "On SRV".to_string(),
            Loadout::OnFoot => "On foot".to_string(),
            Loadout::Unknown => "".to_string(),
        }
    }
}
