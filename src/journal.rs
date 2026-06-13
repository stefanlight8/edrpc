use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use chrono::{TimeDelta, Utc};
use edjr::{AsyncRead, Journal, JournalEvent};
use tokio::{fs::File, sync::mpsc::Sender, time::sleep};

use crate::{message::Message, utils::get_last_journal};

pub struct JournalWatcher {
    tx: Sender<Message>,
    current_path: Option<PathBuf>,
    last_path: Option<PathBuf>,
}

impl JournalWatcher {
    pub fn new(tx: Sender<Message>) -> JournalWatcher {
        JournalWatcher {
            tx,
            current_path: None,
            last_path: None,
        }
    }

    pub async fn run(&mut self, journals_path: PathBuf) -> Result<()> {
        tracing::info!("listening {}", journals_path.display());

        loop {
            let last_journal = get_last_journal(&journals_path)?;

            if self.last_path.as_ref() == Some(&last_journal) {
                sleep(Duration::from_secs(10)).await;
                continue;
            }

            if self.current_path.is_none() || (self.current_path.as_ref() != Some(&last_journal)) {
                tracing::info!("switching to {}", last_journal.display());
                self.current_path = Some(last_journal);
                self.last_path = None;
            }

            let mut journal = Journal::<File>::open(self.current_path.as_ref().unwrap()).await?;
            tracing::debug!("watching journal");
            let entries = journal.read_all().await?;

            if let Some(entry) = entries.first() {
                if let Some(entry) = entries.last() {
                    if Utc::now() - entry.timestamp > TimeDelta::minutes(1) {
                        tracing::debug!(
                            "journal probably dead, because last entry is too old ({}), waiting until new journal",
                            Utc::now() - entry.timestamp
                        );
                        self.last_path = self.current_path.clone();
                        self.current_path = None;
                        continue;
                    }
                }

                if let JournalEvent::Fileheader(_) = &entry.event {
                    self.tx
                        .send(Message::SessionStart {
                            timestamp: entry.timestamp,
                        })
                        .await?;
                } else {
                    continue;
                }

                self.tx
                    .send(Message::JournalEvents(
                        entries.into_iter().map(|entry| entry.event).collect(),
                    ))
                    .await?;

                let mut reader = journal.reader();
                loop {
                    let entries = reader.read_all().await?;

                    if let Some(entry) = entries.last() {
                        if Utc::now() - entry.timestamp > TimeDelta::minutes(5) {
                            self.last_path = self.current_path.clone();
                            self.current_path = None;
                            break;
                        }
                    }

                    for entry in entries {
                        match entry.event {
                            JournalEvent::Shutdown => {
                                tracing::debug!("shutdown received, journal end");
                                self.last_path = self.current_path.clone();
                                self.current_path = None;
                                self.tx.send(Message::SessionEnd).await?
                            }
                            event => self.tx.send(Message::JournalEvent(event)).await?,
                        }
                    }

                    sleep(Duration::from_secs(10)).await;
                }
            }

            sleep(Duration::from_secs(10)).await;
        }
    }
}
