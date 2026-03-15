#[derive(Debug)]
pub enum Loadout {
    OnFoot,
    Ship {
        ship_type: String,
        ship_name: String,
        ship_id: String,
    },
    Srv,
    Unknown,
}
