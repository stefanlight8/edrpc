use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn get_last_journal(dir: impl AsRef<Path>) -> io::Result<PathBuf> {
    fs::read_dir(dir)?
        .filter_map(|e| {
            let entry = e.ok()?;
            let path = entry.path();

            if path.extension()?.to_str()? == "log" {
                Some(path)
            } else {
                None
            }
        })
        .max()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "log file not found"))
}
