use edjr::JournalEvent;

#[derive(Debug, Default, Clone)]
pub enum Location {
    Station {
        station_name: String,
    },
    Body {
        body: String,
    },
    System {
        star_system: String,
    },
    Settlement {
        name: String,
    },
    UssSignal {
        threat: u8,
        uss_type: String,
    },
    #[default]
    Unkonwn,
}

impl Location {
    pub fn update(&mut self, event: &JournalEvent) {
        match event {
            JournalEvent::Location(event) => {
                if let Some(station) = &event.station {
                    *self = Location::Station {
                        station_name: station.station_name.clone(),
                    }
                } else {
                    *self = Location::Body {
                        body: event.body.clone(),
                    }
                }
            }
            JournalEvent::ApproachBody(event) => {
                *self = Location::Body {
                    body: event.body.clone(),
                }
            }
            JournalEvent::ApproachSettlement(event) => {
                *self = Location::Settlement {
                    name: event.name.clone(),
                }
            }
            JournalEvent::SupercruiseEntry(event) => {
                *self = Location::System {
                    star_system: event.star_system.clone(),
                }
            }
            JournalEvent::SupercruiseExit(event) => {
                if !matches!(self, Location::UssSignal { .. }) {
                    *self = Location::Body {
                        body: event.body.clone(),
                    }
                }
            }
            JournalEvent::Docked(event) => {
                if let Some(station) = &event.station {
                    *self = Location::Station {
                        station_name: station.station_name.clone(),
                    }
                }
            }
            JournalEvent::UssDrop(event) => {
                *self = Location::UssSignal {
                    threat: event.uss_threat.clone(),
                    uss_type: event.uss_type.clone(),
                }
            }
            _ => (),
        }
    }
}
