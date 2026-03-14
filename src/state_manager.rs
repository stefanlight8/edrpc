use {
    crate::{
        elite::{self, journal_reader::JournalReader, utils::get_last_journal},
        loadout::Loadout,
        rpc,
        state::GameState,
    },
    anyhow::Result,
    std::{path::PathBuf, sync::Arc},
    tokio::{
        select,
        sync::{Notify, mpsc},
        time::{Duration, sleep},
    },
};

pub struct GameStateManager {
    event_tx: mpsc::Sender<rpc::Event>,
    journals_path: PathBuf,
}

impl GameStateManager {
    pub fn new(event_tx: mpsc::Sender<rpc::Event>, journals_path: PathBuf) -> Self {
        Self {
            event_tx,
            journals_path,
        }
    }

    pub async fn start(&self, shutdown: Arc<Notify>) -> Result<()> {
        self.watchdog(shutdown).await?;

        Ok(())
    }

    async fn watchdog(&self, shutdown: Arc<Notify>) -> Result<()> {
        let mut journal_file: Option<PathBuf> = None;
        let mut journal_reader: Option<JournalReader> = None;

        loop {
            select! {
                _ = sleep(Duration::from_secs(5)) => (),
                _ = shutdown.notified() => {
                    tracing::info!("shutdown received, stopping watchdog");
                    break;
                }
            };

            let last_file = get_last_journal(&self.journals_path)?;
            if Some(&last_file) != journal_file.as_ref() {
                let reader = JournalReader::open(&last_file).await?;
                journal_reader.replace(reader);
                journal_file.replace(last_file);
                tracing::debug!("switch to {:?}", journal_file);
            }

            if let Some(reader) = journal_reader.as_mut() {
                let events = self.listen_journal(reader, shutdown.clone()).await?;

                for event in events {
                    self.event_tx.send(event).await?;
                }
            }
        }

        Ok(())
    }

    async fn listen_journal(
        &self,
        reader: &mut JournalReader,
        shutdown: Arc<Notify>,
    ) -> Result<Vec<rpc::Event>> {
        let entries = select! {
            entries = reader.poll() => entries,
            _ = shutdown.notified() => {
                tracing::info!("shutdown received, stopping listen_journal");
                return Ok(vec![]);
            },
        }?;
        let mut events: Vec<rpc::Event> = Vec::with_capacity(entries.len());

        for entry in entries {
            match entry.event {
                elite::events::Event::ApproachBody { body, .. } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Approaching(body)))
                }
                elite::events::Event::ApproachSettlement {
                    name,
                    name_localised,
                    ..
                } => events.push(rpc::Event::GameStateUpdate(GameState::Approaching(
                    name_localised.unwrap_or(name),
                ))),
                elite::events::Event::CarrierJump { on_foot: true, .. } => {
                    events.push(rpc::Event::LoadoutUpdate(Loadout::OnFoot))
                }
                elite::events::Event::CarrierJump { .. } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::OnCarrier))
                }
                elite::events::Event::Disembark {
                    body,
                    on_srv,
                    station_name,
                    ..
                } => {
                    if on_srv {
                        events.push(rpc::Event::LoadoutUpdate(Loadout::Srv))
                    } else {
                        events.push(rpc::Event::LoadoutUpdate(Loadout::OnFoot))
                    }

                    events.push(rpc::Event::GameStateUpdate(GameState::Location(
                        station_name.unwrap_or(body),
                    )))
                }
                elite::events::Event::Docked { station_name, .. } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Docked(station_name)))
                }
                elite::events::Event::FSDJump { star_system, .. } => events.push(
                    rpc::Event::GameStateUpdate(GameState::Supercruise(Some(star_system))),
                ),
                elite::events::Event::LeaveBody { body, .. } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Location(body)));
                }
                elite::events::Event::Liftoff {
                    body: Some(body), ..
                } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Location(body)));
                }
                elite::events::Event::Liftoff {
                    star_system: Some(star_system),
                    ..
                } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Location(
                        star_system,
                    )));
                }
                elite::events::Event::Liftoff { .. } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Location(
                        "Unknown".to_string(),
                    )));
                }
                elite::events::Event::LoadGame {
                    ship: Some(ship),
                    ship_ident: Some(ship_ident),
                    ship_name: Some(ship_name),
                    ..
                }
                | elite::events::Event::Loadout {
                    ship,
                    ship_ident,
                    ship_name,
                } => {
                    events.push(rpc::Event::LoadoutUpdate(Loadout::Ship {
                        ship_type: ship,
                        ship_name: ship_name,
                        ship_id: ship_ident,
                    }));
                }
                elite::events::Event::LoadGame {
                    start_dead: Some(true),
                    ..
                } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Dead));
                }
                elite::events::Event::LoadGame {
                    start_landed: Some(true),
                    ..
                } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Landed(None)));
                }
                elite::events::Event::LoadGame { .. } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Idle));
                }
                elite::events::Event::Location {
                    docked: true,
                    station_name: Some(station_name),
                    ..
                } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Docked(station_name)));
                }
                elite::events::Event::Location {
                    station_name: Some(station_name),
                    ..
                } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Docked(station_name)));
                }
                elite::events::Event::Location {
                    body, docked: true, ..
                } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Landed(Some(body))));
                }
                elite::events::Event::Location { on_foot: true, .. } => {
                    events.push(rpc::Event::LoadoutUpdate(Loadout::OnFoot));
                }
                elite::events::Event::Shutdown => {} // somehow make to rpc disable
                elite::events::Event::SupercruiseEntry { star_system } => events.push(
                    rpc::Event::GameStateUpdate(GameState::Supercruise(Some(star_system))),
                ),
                elite::events::Event::SupercruiseDestinationDrop {
                    destination,
                    destination_localized,
                } => events.push(rpc::Event::GameStateUpdate(GameState::Location(
                    destination_localized.unwrap_or(destination),
                ))),
                // elite::events::Event::SupercruiseExit { body, .. } => {
                //    events.push(rpc::Event::GameStateUpdate(GameState::Location(body)));
                //}
                elite::events::Event::Touchdown {
                    nearest_destination: Some(nearest_destination),
                    nearest_localised,
                    ..
                } => events.push(rpc::Event::GameStateUpdate(GameState::Landed(Some(
                    nearest_localised.unwrap_or(nearest_destination),
                )))),
                elite::events::Event::Touchdown {
                    body: Some(body), ..
                } => events.push(rpc::Event::GameStateUpdate(GameState::Landed(Some(body)))),
                elite::events::Event::USSDrop { name } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Location(name)));
                }
                elite::events::Event::Undocked { station_name } => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Location(
                        station_name,
                    )));
                }
                elite::events::Event::Died => {
                    events.push(rpc::Event::GameStateUpdate(GameState::Dead))
                }
                elite::events::Event::StartJump {
                    star_system: Some(star_system),
                    ..
                } => events.push(rpc::Event::GameStateUpdate(GameState::JumpingTo(
                    star_system,
                ))),
                elite::events::Event::Unknown => {}
                unhandled => {
                    tracing::trace!("unhandled event: {:?}", unhandled);
                }
            };
        }

        Ok(events)
    }
}
