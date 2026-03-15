#[derive(Debug)]
pub enum GameState {
    Approaching(String),
    Dead,
    Docked(String),
    Idle,
    JumpingTo(String),
    Landed(Option<String>),
    Location(String),
    OnCarrier,
    Supercruise(Option<String>),
}
