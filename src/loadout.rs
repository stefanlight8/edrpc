use edjr::JournalEvent;

#[derive(Debug, Default, Clone)]
pub enum Loadout {
    Ship {
        ship_type: String,
        ship_name: Option<String>,
    },
    Srv,
    Suit {
        suit_type: String,
    },
    #[default]
    Unknown,
}

impl Loadout {
    pub fn update(&mut self, event: &JournalEvent) {
        match event {
            JournalEvent::Loadout(event) => {
                let ship = &event.ship;

                *self = Loadout::Ship {
                    ship_type: ship.ship.clone(),
                    ship_name: ship.ship_name.clone(),
                }
            }
            JournalEvent::SuitLoadout(event) => {
                *self = Loadout::Suit {
                    suit_type: event.suit_name.clone(),
                }
            }
            JournalEvent::Disembark(event) if event.srv => {
                *self = Loadout::Srv;
            }
            _ => (),
        }
    }
}
