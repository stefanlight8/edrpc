use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use chrono::{TimeDelta, Utc};
use edjr::{AsyncRead, Journal, JournalEntry, JournalEvent};
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

        let mut current_path = None;
        let mut last_path = None;
        let mut reader = None;

        loop {
            let last_journal = get_last_journal(&journals_path)?;

            if Some(&last_journal) == last_path.as_ref() {
                sleep(Duration::from_secs(5)).await;
                continue;
            }

            if current_path.as_ref() != Some(&last_journal) {
                tracing::info!("switching to {}", last_journal.display());

                let mut journal = Journal::<File>::open(&last_journal).await?;
                let entries = journal.read_all().await?;

                let Some(JournalEntry {
                    timestamp,
                    event: JournalEvent::Fileheader(_),
                }) = entries.first()
                else {
                    sleep(Duration::from_secs(5)).await;
                    continue;
                };

                if let Some(JournalEntry {
                    timestamp: _,
                    event: JournalEvent::Shutdown,
                }) = entries.last()
                {
                    last_path = Some(last_journal);
                    continue;
                }

                self.tx
                    .send(Message::SessionStart {
                        timestamp: *timestamp,
                    })
                    .await?;

                reader = Some(journal.reader());
                current_path = Some(last_journal);
            }

            if let Some(current_reader) = reader.as_mut() {
                for entry in current_reader.read_all().await? {
                    match entry.event {
                        JournalEvent::Shutdown => {
                            tracing::debug!("shutdown received, journal end");
                            self.tx.send(Message::SessionEnd).await?;
                            last_path = current_path.take();
                            reader = None;
                            break;
                        }
                        event => self.tx.send(Message::JournalEvent(event)).await?,
                    }
                }
            }

            sleep(Duration::from_secs(5)).await;
        }
    }
}
