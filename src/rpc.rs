use std::{path::PathBuf, process::id};

use anyhow::Result;
use daito::{
    api::activity::{Activity, ActivityAssets, ActivityTimestamps},
    rpc::client::RpcClient,
};

use crate::{loadout::Loadout, message::Message, session::GameSession, state::GameState};

pub struct Rpc {
    pid: u32,
    client: RpcClient,
    game_session: Option<GameSession>,
}

impl Rpc {
    pub async fn new(client_id: String, pipe_path: PathBuf) -> Result<Rpc> {
        let client = RpcClient::open(client_id, pipe_path).await?;

        Ok(Rpc {
            client,
            pid: id(),
            game_session: None,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        self.client.handshake().await?;

        Ok(())
    }

    pub async fn update(&mut self, message: Message) -> Result<()> {
        match message {
            Message::SessionStart { timestamp } => {
                self.game_session = Some(GameSession::new(timestamp))
            }
            Message::SessionEnd => self.game_session = None,
            Message::JournalEvent(event) if let Some(game_session) = &mut self.game_session => {
                game_session.update(event);
            }
            Message::JournalEvents(events) if let Some(game_session) = &mut self.game_session => {
                for event in events {
                    game_session.update(event);
                }
            }
            _ => (),
        }

        self.update_activity().await?;

        Ok(())
    }

    async fn update_activity(&mut self) -> Result<()> {
        if self.game_session.is_none() {
            self.client.clear_activity(self.pid.clone()).await?;
        } else if let Some(game_session) = self.game_session.clone() {
            let mut activity = Activity {
                name: "Elite Dangerous".to_string(),
                assets: Some(ActivityAssets {
                    large_image: Some("elite-dangerous".to_string()),
                    ..Default::default()
                }),
                timestamps: Some(ActivityTimestamps {
                    start: Some(game_session.timestamp.timestamp().into()),
                    end: None,
                }),
                ..Default::default()
            };

            match game_session.loadout {
                Loadout::Ship {
                    ship_type,
                    ship_name,
                } => {
                    activity.assets = Some(ActivityAssets {
                        large_image: Some("ship".to_string()),
                        large_text: Some(format!("{}", ship_type)),
                        small_image: Some("elite-dangerous-minimalistic".to_string()),
                        small_text: Some("Elite Dangerous".to_string()),
                        ..Default::default()
                    })
                }
                Loadout::Srv => {
                    activity.assets = Some(ActivityAssets {
                        large_image: Some("srv".to_string()),
                        small_image: Some("elite-dangerous-minimalistic".to_string()),
                        small_text: Some("Elite Dangerous".to_string()),
                        ..Default::default()
                    })
                }
                Loadout::Suit { suit_type } => {
                    activity.assets = Some(ActivityAssets {
                        large_image: Some("srv".to_string()),
                        large_text: Some(format!("{}", suit_type)),
                        small_image: Some("elite-dangerous-minimalistic".to_string()),
                        small_text: Some("Elite Dangerous".to_string()),
                        ..Default::default()
                    })
                }
                _ => (),
            }

            match game_session.state {
                GameState::InGame => {
                    activity.state = Some("In game".to_string());
                }
                _ => (),
            }

            self.client.set_activity(self.pid.clone(), activity).await?;
        }

        Ok(())
    }
}
