use edjr::JournalEvent;

#[derive(Debug, Default, Clone)]
pub enum GameState {
    #[default]
    InGame,
    Dead,
    Docked,
    Landed,
    Body,
    DeepSpace,
    Supercruise,
}

impl GameState {
    pub fn update(&mut self, event: &JournalEvent) {
        match event {
            JournalEvent::Docked(_) => {
                *self = GameState::Docked;
            }
            JournalEvent::Undocked(_) => {
                *self = GameState::Body;
            }
            JournalEvent::Touchdown(_) => {
                *self = GameState::Landed;
            }
            JournalEvent::Liftoff(_) => {
                *self = GameState::Body;
            }
            JournalEvent::SupercruiseEntry(_) => {
                *self = GameState::Supercruise;
            }
            JournalEvent::SupercruiseExit(_) => {
                *self = GameState::DeepSpace;
            }
            JournalEvent::LoadGame(event) => {
                if event.start_dead {
                    *self = GameState::Dead
                } else if event.start_landed {
                    *self = GameState::Landed
                }
            }
            JournalEvent::Location(event) => {
                if event.docked {
                    *self = GameState::Docked
                }
            }
            _ => (),
        }
    }
}
