use {
    crate::{loadout::Loadout, state::GameState},
    chrono::{DateTime, Local},
};

pub enum Event {
    SessionUpdate { created_at: DateTime<Local> },
    GameStateUpdate(GameState),
    LoadoutUpdate(Loadout),
}
