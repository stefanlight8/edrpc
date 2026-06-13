use chrono::{DateTime, Utc};
use edjr::JournalEvent;

#[derive(Debug)]
pub enum Message {
    SessionStart { timestamp: DateTime<Utc> },
    SessionEnd,
    JournalEvent(JournalEvent),
    JournalEvents(Vec<JournalEvent>),
}
