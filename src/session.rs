use chrono::{DateTime, Utc};
use edjr::JournalEvent;

use crate::{loadout::Loadout, location::Location, state::GameState};

#[derive(Debug, Clone)]
pub struct GameSession {
    pub state: GameState,
    pub location: Location,
    pub loadout: Loadout,
    pub timestamp: DateTime<Utc>,
}

impl GameSession {
    pub fn new(timestamp: DateTime<Utc>) -> GameSession {
        GameSession {
            timestamp,
            state: GameState::default(),
            location: Location::default(),
            loadout: Loadout::default(),
        }
    }

    pub fn update(&mut self, event: JournalEvent) {
        match event {
            event => {
                self.state.update(&event);
                self.location.update(&event);
                self.loadout.update(&event)
            }
        }
    }
}
