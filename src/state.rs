use edjr::JournalEvent;

#[derive(Debug, Default, Clone)]
pub enum GameState {
    #[default]
    InGame,
    DeepSpace {
        star_system: String,
    },
    Supercruise {
        star_system: String,
    },
    UssSignal {
        uss_type: String,
        uss_threat: String,
    },
    Body {
        body: String,
    },
    Station {
        station: String,
    },
}

impl GameState {
    pub fn update(&mut self, event: &JournalEvent) {}
}
