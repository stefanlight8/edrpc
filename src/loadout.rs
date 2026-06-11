pub enum Loadout {
    Ship {
        ship_type: String,
        ship_name: Option<String>,
    },
    Srv {
        srv_type: String,
    },
    Suit {
        suit_type: String,
    },
}
