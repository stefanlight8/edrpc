use {
    crate::{
        elite::{
            events::{Event, JumpType},
            journal_reader::JournalReader,
            utils::get_last_journal,
        },
        loadout::Loadout,
        message::Message,
        state::GameState,
    },
    anyhow::Result,
    std::{path::PathBuf, time::Duration},
    tokio::{sync::mpsc::Sender, time::sleep},
};

fn event_to_message(event: Event) -> Option<Message> {
    match event {
        Event::ApproachBody { body, .. } => Some(Message::Update {
            state: Some(GameState::Approaching(body)),
            loadout: None,
        }),
        Event::ApproachSettlement {
            name,
            name_localised,
            ..
        } => Some(Message::Update {
            state: Some(GameState::Approaching(name_localised.unwrap_or(name))),
            loadout: None,
        }),
        Event::CarrierJump { on_foot, .. } => Some(Message::Update {
            state: Some(GameState::OnCarrier),
            loadout: Some(Loadout::OnFoot).filter(|_| on_foot),
        }),
        Event::Disembark {
            body,
            on_srv,
            station_name,
            ..
        } => {
            let loadout = if on_srv {
                Loadout::Srv
            } else {
                Loadout::OnFoot
            };

            Some(Message::Update {
                state: Some(GameState::Location(station_name.unwrap_or(body))),
                loadout: Some(loadout),
            })
        }
        Event::LeaveBody { star_system, .. } => Some(Message::Update {
            state: Some(GameState::Supercruise(Some(star_system))),
            loadout: None,
        }),
        Event::Liftoff {
            body: Some(body), ..
        } => Some(Message::Update {
            state: Some(GameState::Location(body)),
            loadout: None,
        }),
        Event::LoadGame {
            ship: Some(ship),
            ship_ident: Some(ship_id),
            ship_name: Some(ship_name),
            start_dead,
            start_landed,
        } => {
            let state = if start_dead.unwrap_or(false) {
                GameState::Dead
            } else if start_landed.unwrap_or(false) {
                GameState::Landed(None)
            } else {
                GameState::Idle
            };

            Some(Message::Update {
                state: Some(state),
                loadout: Some(Loadout::Ship {
                    ship_type: ship,
                    ship_id,
                    ship_name,
                }),
            })
        }
        // TODO(LoadGame): determine loadout from ship, because ship is not only about ship,
        // but about suit/srv too. Like $test_buggy; is a scarab.
        Event::Location {
            body,
            docked,
            on_foot,
            station_name,
            ..
        } => Some(if docked && let Some(station_name) = station_name {
            Message::Update {
                state: Some(GameState::Docked(station_name)),
                loadout: None,
            }
        } else if on_foot {
            Message::Update {
                state: Some(GameState::Location(station_name.unwrap_or(body))),
                loadout: Some(Loadout::OnFoot),
            }
        } else {
            Message::Update {
                state: Some(GameState::Location(body)),
                loadout: None,
            }
        }),
        Event::Loadout {
            ship,
            ship_ident,
            ship_name,
        } => Some(Message::Update {
            state: None,
            loadout: Some(Loadout::Ship {
                ship_type: ship,
                ship_name,
                ship_id: ship_ident,
            }),
        }),
        Event::Shutdown => None, // TODO(Shutdown): Message::Shutdown
        Event::SupercruiseEntry { star_system } => Some(Message::Update {
            state: Some(GameState::Supercruise(Some(star_system))),
            loadout: None,
        }),
        Event::SupercruiseDestinationDrop {
            destination,
            destination_localized,
        } => Some(Message::Update {
            state: Some(GameState::Location(
                destination_localized.unwrap_or(destination),
            )),
            loadout: None,
        }),
        Event::Touchdown {
            body,
            nearest_destination,
            nearest_localised,
            star_system,
        } => {
            let location: Option<String> = nearest_localised
                .or(nearest_destination)
                .or(body)
                .or(star_system);

            Some(Message::Update {
                state: Some(GameState::Landed(location)),
                loadout: None,
            })
        }
        Event::USSDrop { name } => Some(Message::Update {
            state: Some(GameState::Location(name)),
            loadout: None,
        }),
        Event::Undocked { station_name } => Some(Message::Update {
            state: Some(GameState::Location(station_name)),
            loadout: None,
        }),
        Event::Died => Some(Message::Update {
            state: Some(GameState::Dead),
            loadout: None,
        }),
        Event::StartJump {
            jump_type: JumpType::Supercruise,
            star_system: Some(star_system),
        } => Some(Message::Update {
            state: Some(GameState::Supercruise(Some(star_system))),
            loadout: None,
        }),
        Event::StartJump {
            jump_type: JumpType::Hyperspace,
            star_system: Some(star_system),
        } => Some(Message::Update {
            state: Some(GameState::JumpingTo(star_system)),
            loadout: None,
        }),

        Event::Unknown => None,
        unhandled => {
            tracing::trace!("[warning] unhandled event variant: {:?}", unhandled);

            None
        }
    }
}

pub async fn watchdog(journals_path: PathBuf, message_tx: Sender<Message>) -> Result<()> {
    let mut journal_path = get_last_journal(&journals_path)?;
    let mut journal_reader = JournalReader::open(&journal_path).await?;
    tracing::debug!("polling {:?}", journals_path);

    loop {
        for entry in journal_reader.poll().await? {
            if let Some(message) = event_to_message(entry.event) {
                if message_tx.send(message).await.is_err() {
                    tracing::info!("message channel is closed, stopping");

                    return Ok(());
                };
            }
        }

        let last_journal = get_last_journal(&journals_path)?;
        if journal_path != last_journal {
            journal_path = last_journal;
            journal_reader = JournalReader::open(&journal_path).await?;

            tracing::debug!("switch to {:?}", journal_path);
        }

        sleep(Duration::from_secs(5)).await;
    }
}
