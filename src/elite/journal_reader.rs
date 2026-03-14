use {
    super::events::JournalEntry,
    std::{
        io::{Error, SeekFrom},
        path::Path,
    },
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, AsyncSeekExt, BufReader},
    },
};

pub struct JournalReader {
    reader: BufReader<File>,
    offset: u64,
}

impl JournalReader {
    pub async fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path).await?;
        let reader = BufReader::new(file);

        Ok(Self { reader, offset: 0 })
    }

    pub async fn read_all(&mut self) -> Result<Vec<JournalEntry>, Error> {
        let mut entries: Vec<JournalEntry> = Vec::new();

        let reader = &mut self.reader;
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            let entry: JournalEntry = serde_json::from_str(line.trim_end())?;

            entries.push(entry);
        }

        Ok(entries)
    }

    pub async fn read_first(&mut self) -> Result<JournalEntry, Error> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;
        let entry = serde_json::from_str(&line)?;
        Ok(entry)
    }

    pub async fn poll(&mut self) -> Result<Vec<JournalEntry>, Error> {
        self.reader.seek(SeekFrom::Start(self.offset)).await?;

        let mut entries: Vec<JournalEntry> = Vec::new();
        let mut line = String::new();

        loop {
            line.clear();

            let bytes = self.reader.read_line(&mut line).await?;
            if bytes == 0 {
                break;
            }

            self.offset += bytes as u64;

            let event: JournalEntry = match serde_json::from_str(line.trim_end()) {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            entries.push(event);
        }

        Ok(entries)
    }
}
