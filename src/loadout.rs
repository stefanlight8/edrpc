use edjr::JournalEvent;

#[derive(Debug, Default, Clone)]
pub enum Loadout {
    Ship {
        ship_type: String,
        ship_name: Option<String>,
    },
    Srv,
    Suit {
        suit_type: Option<String>,
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
                    suit_type: Some(event.suit_name.clone()),
                }
            }
            JournalEvent::Disembark(event) => {
                if event.in_srv {
                    *self = Loadout::Srv;
                } else {
                    *self = Loadout::Suit { suit_type: None }
                }
            }
            JournalEvent::Location(event) => {
                if event.in_srv {
                    *self = Loadout::Srv;
                } else if event.on_foot {
                    *self = Loadout::Suit { suit_type: None }
                }
            }
            _ => (),
        }
    }
}
