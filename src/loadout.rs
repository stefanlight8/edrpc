pub enum Loadout {
    Ship {
        ship_type: String,
        ship_name: String,
        ship_id: String,
    },
    Srv,
    OnFoot,
    Unknown,
}
