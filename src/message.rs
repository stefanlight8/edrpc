use crate::{loadout::Loadout, state::GameState};

#[derive(Debug)]
pub enum Message {
    Session {
        created_at: u64,
    },
    Update {
        state: Option<GameState>,
        loadout: Option<Loadout>,
    },
}
