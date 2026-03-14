pub enum GameState {
    Idle,
    Supercruise(Option<String>),
    Location(String),
    Docked(String),
    Approaching(String),
    OnCarrier,
    Dead,
    Landed(Option<String>),
}
